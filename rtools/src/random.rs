use std::ops::Range;

use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::SliceRandom,
    thread_rng, Rng,
};

pub trait Random<T = Self>: Sized {
    fn random() -> T {
        unimplemented!()
    }
    fn random_in(_: Range<Self>) -> T {
        unimplemented!()
    }
    fn random_member(&self) -> &T {
        unimplemented!()
    }
}

impl Random for bool {
    fn random() -> Self {
        let mut rng = thread_rng();
        rng.gen_range(0..=1) != 0
    }
}

impl Random for char {
    fn random() -> Self {
        thread_rng().gen()
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for String {
    fn random() -> Self {
        let mut rng = thread_rng();
        Alphanumeric.sample_string(&mut rng, 8)
    }
}

impl Random for u8 {
    fn random() -> Self {
        thread_rng().gen()
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for i32 {
    fn random() -> Self {
        thread_rng().gen_range(-100..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for u64 {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for i64 {
    fn random() -> Self {
        thread_rng().gen_range(-100..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for u32 {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for isize {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for usize {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for f32 {
    fn random() -> Self {
        thread_rng().gen_range(0.0..100.0)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for f64 {
    fn random() -> Self {
        thread_rng().gen_range(0.0..100.0)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl<T> Random<T> for Vec<T> {
    fn random_member(&self) -> &T {
        self.choose(&mut thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::Random;

    #[test]
    fn random_int() {
        for _ in 0..100 {
            let rang = 5..100;
            let ran = u32::random_in(rang.clone());
            assert!(rang.contains(&ran));
        }
    }

    #[test]
    fn random_vec_elem() {
        for _ in 0..100 {
            let ve = vec![1, 2, 3, 4, 5];
            let r = ve.random_member();
            assert!(ve.contains(r));
        }
    }
}