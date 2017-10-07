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

use uuid::Uuid;

use serde::Serialize;
use serde::de::DeserializeOwned;

use reqwest::header::Authorization;
use reqwest::{IntoUrl, StatusCode, Method, Response};

use errors::*;
use account::Account;
use pomo::{Pomo, PomoParameter};
use todo::{Todo, SubTodo, TodoParameter};

const TODO_URL: &'static str = "https://api.pomotodo.com/1/todos";
const POMO_URL: &'static str = "https://api.pomotodo.com/1/pomos";
const INFO_URL: &'static str = "https://api.pomotodo.com/1/account";

/// A `Client` to communicate with Pomotodo server.
///
/// The `Client` holds the access token and a `reqwest::Client`.
///
/// # Example
///
/// ```rust
/// # use pomotodo::{Client, PomoParameter};
/// #
/// # fn run() {
/// let client = Client::new("YOUR_ACCESS_TOKEN");
/// let pomos = client.pomos(PomoParameter::default());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    token: String,
    inner: ::reqwest::Client,
}

impl Default for Client {
    fn default() -> Client {
        Client {
            token: String::new(),
            inner: ::reqwest::Client::new(),
        }
    }
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new<T>(token: T) -> Client
        where T: Into<String>
    {
        Client {
            token: token.into(),
            ..Default::default()
        }
    }

    /// Request for the [`Account`](struct.Account.html)'s profile.
    pub fn account(&self) -> Result<Account, Error> {
        self.get(INFO_URL)
    }

    /// Request for the [`Pomo`](struct.Pomo.html) specified by `uuid`.
    pub fn pomo<U: Into<Uuid>>(&self, uuid: U) -> Result<Pomo, Error> {
        let url = format!("{}/{}", POMO_URL, uuid.into());
        self.get(url.as_str())
    }

    /// Request for all [`Pomo`](struct.Pomo.html) that matched the `param`.
    pub fn pomos(&self, param: PomoParameter) -> Result<Vec<Pomo>, Error> {
        let query = param.to_query();
        let url = if !query.is_empty() {
            POMO_URL.to_owned()
        } else {
            format!("{}?{}", POMO_URL, query)
        };

        self.get(url.as_str())
    }

    /// Submit a new [`Pomo`](struct.Pomo.html) to server.
    pub fn submit_pomo(&self, pomo: &Pomo) -> Result<Pomo, Error> {
        self.post(POMO_URL, pomo)
    }

    // TODO: The parameter of the patch (update) should do more test
    pub fn update_pomo<U, S>(&self, _: U, _: S) -> Result<Pomo, Error>
        where U: Into<Uuid>, S: Into<String>
    {
        unimplemented!();
    }

    /// Requests server to delete the [`Pomo`](struct.Pomo.html) specified by `uuid`.
    pub fn delete_pomo<U: Into<Uuid>>(&self, uuid: U) -> Result<(), Error> {
        let url = format!("{}/{}", POMO_URL, uuid.into());
        self.delete(url.as_str())
    }

    /// Request for the [`Todo`](struct.Todo.html) specified by `uuid`.
    pub fn todo<U: Into<Uuid>>(&self, uuid: U) -> Result<Todo, Error> {
        let url = format!("{}/{}", TODO_URL, uuid.into());
        self.get(url.as_str())
    }

    /// Request for all [`Todo`](struct.Todo.html) that matched the `param`.
    pub fn todos(&self, param: TodoParameter) -> Result<Vec<Todo>, Error> {
        let query = param.to_query();
        let url = if !query.is_empty() {
            TODO_URL.to_owned()
        } else {
            format!("{}?{}", TODO_URL, query)
        };

        self.get(url.as_str())
    }

    /// Requests server to creates a new [`Todo`](struct.Todo.html).
    pub fn create_todo(&self, todo: &Todo) -> Result<Todo, Error> {
        self.post(TODO_URL, todo)
    }

    // TODO: The parameter of the patch (update) should do more test
    pub fn update_todo<U: Into<Uuid>>(&self, _: U, _: &Todo) -> Result<Todo, Error> {
        unimplemented!();
    }

    /// Requests server to delete the [`Todo`](struct.Todo.html) specified by `uuid`.
    pub fn delete_todo<U: Into<Uuid>>(&self, uuid: U) -> Result<(), Error> {
        let url = format!("{}/{}", TODO_URL, uuid.into());
        self.delete(url.as_str())
    }

    /// Request for the [`SubTodo`](struct.SubTodo.html)
    /// owned by `parent` and had the `uuid`.
    pub fn subtodo<U: Into<Uuid>>(&self, parent: U, uuid: U) -> Result<SubTodo, Error> {
        let url = format!("{}/{}/sub_todos/{}", TODO_URL, parent.into(), uuid.into());
        self.get(url.as_str())
    }

    /// Request for all [`SubTodo`](struct.SubTodo.html) owned by `parent`.
    pub fn subtodos<U: Into<Uuid>>(&self, parent: U) -> Result<Vec<SubTodo>, Error> {
        let url = format!("{}/{}/sub_todos", TODO_URL, parent.into());
        self.get(url.as_str())
    }

    /// Requests server to create a new [`SubTodo`](struct.SubTodo.html)
    /// under the [`Todo`](struct.Todo.html) specified by `parent`.
    pub fn create_subtodo<U: Into<Uuid>>(
        &self,
        parent: U,
        sub_todo: &SubTodo,
    ) -> Result<SubTodo, Error> {
        let url = format!("{}/{}/sub_todos", TODO_URL, parent.into());
        self.post(url.as_str(), sub_todo)
    }

    // TODO: The parameter of the patch (update) should do more test
    pub fn update_subtodo<U: Into<Uuid>>(&self, _: U, _: U, _: &SubTodo) -> Result<SubTodo, Error> {
        unimplemented!();
    }

    /// Requests server to delete the [`SubTodo`](struct.Todo.html)
    /// owned by `parent`and had the `uuid`.
    pub fn delete_subtodo<U: Into<Uuid>>(&self, parent: U, uuid: U) -> Result<(), Error> {
        let url = format!("{}/{}/sub_todos/{}", TODO_URL, parent.into(), uuid.into());
        self.delete(url.as_str())
    }

    /// An wrap of `reqwest::Client::request` to make request with json body.
    fn request<U, T>(&self, method: Method, url: U, json: Option<&T>) -> Result<Response, Error>
        where U: IntoUrl, T: Serialize + DeserializeOwned
    {
        let mut request = self.inner.request(method, url);
        if let Some(json) = json {
            request.json(json);
        }

        request
            .header(Authorization(format!("token {}", self.token)))
            .send()
            .and_then(|resp| resp.error_for_status())
            .map_err(|e| e.into())
            .and_then(|resp| if resp.status() == StatusCode::Ok {
                          Ok(resp)
                      } else {
                          Err(Error::from(ErrorKind::Unexcept("server redirect".to_owned())))
                      })
    }

    /// Convenience method to make a POST request with json body to a URL.
    fn post<U, T>(&self, url: U, json: &T) -> Result<T, Error>
        where U: IntoUrl, T: Serialize + DeserializeOwned
    {
        self.request(Method::Post, url, Some(json))
            .and_then(|mut resp| resp.json().map_err(|e| e.into()))
    }

    /// Convenience method to make a GET request body to a URL.
    fn get<U, T>(&self, url: U) -> Result<T, Error>
        where U: IntoUrl, T: Serialize + DeserializeOwned
    {
        self.request::<_, T>(Method::Get, url, None)
            .and_then(|mut resp| resp.json().map_err(|e| e.into()))
    }

    /// Convenience method to make a DELETE request to a URL.
    fn delete<U: IntoUrl>(&self, url: U) -> Result<(), Error> {
        self.request::<_, ()>(Method::Delete, url, None).and_then(|_| Ok(()))
    }
}
