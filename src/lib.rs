#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate chrono;
extern crate hyper_native_tls;
extern crate uuid;

pub mod account;
pub mod pomo;
pub mod todo;
pub mod session;

trait ShouldSkip: Sized {
    fn should_skip(&self) -> bool;
}

impl<T> ShouldSkip for Option<T> {
    fn should_skip(&self) -> bool {
        self.is_none()
    }
}
