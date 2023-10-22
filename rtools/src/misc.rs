use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread,
    time::Duration,
};

use backtrace::Backtrace;

use crate::IntoF32;

pub fn backtrace() {
    let bt = Backtrace::new();
    println!("{bt:?}");
    error!("{bt:?}");
}

pub fn hash(obj: impl ToString + Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub fn sleep(duration: impl IntoF32) {
    thread::sleep(Duration::from_nanos((duration.into_f32() * 1000000000.0) as _));
}

pub trait Toggle {
    fn toggle(&mut self) -> bool;
}

/// Returns old value
impl Toggle for bool {
    fn toggle(&mut self) -> bool {
        *self = !*self;
        !*self
    }
}

#[cfg(test)]
mod test {

    use crate::{hash, passed::Passed, random::Random, sleep, Toggle};

    #[test]
    fn toggle() {
        let mut val = bool::random();

        for _ in 0..10 {
            let prev = val;
            assert_eq!(val.toggle(), prev);
            assert_eq!(val, !prev);
        }
    }

    #[test]
    fn misc() {
        assert_eq!(397663693774528972, hash(54325));
        assert_eq!(13713634450856707654, hash(8657865));
        assert_eq!(5949921715258702887, hash(87));
    }

    #[tokio::test]
    async fn test_sleep() {
        let mut pass = Passed::default();

        sleep(0.2);
        let passed = pass.passed();
        assert!(passed >= 200);
        assert!(passed < 220);
    }
}
