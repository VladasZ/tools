#![allow(incomplete_features)]
#![allow(clippy::mismatched_target_os)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_for)]
#![feature(const_default_impls)]

#[macro_use]
extern crate log;
extern crate core;

pub mod address;
pub mod animation;
pub mod array_view;
pub mod bytes;
pub mod data;
pub mod data_manager;
mod dispatch;
pub mod event;
pub mod file;
mod logger;
pub mod math;
mod misc;
pub mod passed;
pub mod paths;
pub mod persistent;
pub mod platform;
mod property;
mod random;
pub mod regex;
pub mod rglica;
pub mod run;
mod selectable;
mod static_get;
pub mod test;
mod time;
pub mod unwrap;

pub use animation::Animation;
pub use dispatch::Dispatch;
pub use event::Event;
pub use logger::init_log;
pub use math::{mm_to_inch, IntoF32};
pub use misc::{backtrace, *};
pub use persistent::Persistent;
pub use property::Property;
pub use random::Random;
pub use rglica::{Rglica, ToRglica};
pub use selectable::Selectable;
pub use time::Time;
pub use unwrap::Unwrap;

pub trait Remove<T> {
    fn remove(&mut self, val: &T);
}

impl<T: PartialEq> Remove<T> for Vec<T> {
    fn remove(&mut self, val: &T) {
        if let Some(pos) = self.iter().position(|a| a == val) {
            self.remove(pos);
        } else {
            info!("Nothing to remove");
        }
    }
}
