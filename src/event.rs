use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    ops::DerefMut,
};

use crate::{refs::to_weak::ToWeak, Strong, UnwrapBox};
pub struct Event<T = ()> {
    subscriber: RefCell<UnwrapBox<dyn FnMut(T) + 'static>>,
}

impl<T: 'static> Event<T> {
    pub fn link(&self, event: &Self) {
        let event = event.weak();
        self.sub(move |val| event.trigger(val));
    }

    pub fn sub(&self, action: impl FnMut(T) + 'static) {
        self.subscriber
            .replace(UnwrapBox::from_box(Strong::new(action)));
    }

    pub fn set<Obj: 'static>(&self, obj: &Obj, mut action: impl FnMut(&mut Obj, T) + 'static) {
        let mut rglica = obj.weak();
        self.subscriber
            .replace(UnwrapBox::from_box(Strong::new(move |value| {
                action(rglica.deref_mut(), value);
            })));
    }

    pub fn trigger(&self, value: T) {
        let mut sub = self.subscriber.borrow_mut();
        if sub.is_null() {
            return;
        }
        sub(value);
    }

    pub fn unsubscribe(&self) {
        self.subscriber.replace(Default::default());
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber: Default::default(),
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event<{}>", std::any::type_name::<T>(),)
    }
}
