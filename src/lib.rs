// Copyright 2017 Kam Y. Tse
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(warnings)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate uuid;
extern crate serde;
extern crate chrono;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

mod account;
mod pomo;
mod todo;
mod client;

pub use self::account::Account;
pub use self::pomo::{Pomo, PomoBuilder, PomoParameter};
pub use self::todo::{Todo, SubTodo, TodoBuilder, SubTodoBuilder, TodoParameter};
pub use self::client::Client;

/// The Errors that may occur when communicating with Pomotodo server.
pub mod errors {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt;
        }

        foreign_links {
            ReqError(::reqwest::Error);
        }
    }
}
