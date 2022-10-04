use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
};

pub struct Event<T = ()> {
    subscriber: RefCell<Option<Box<dyn FnMut(T) + 'static>>>,
}

impl<T: 'static> Event<T> {
    pub fn sub(&self, action: impl FnMut(T) + 'static) {
        self.subscriber.replace(Some(Box::new(action)));
    }

    pub fn trigger(&self, value: T) {
        let mut sub = self.subscriber.borrow_mut();
        if sub.is_none() {
            return;
        }
        (sub.as_mut().unwrap())(value);
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
