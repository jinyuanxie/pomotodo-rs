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
use chrono::prelude::*;

/// The repeat type of [`Todo`](struct.Todo.html).
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatType {
    None,
    EachDay,
    EachWeek,
    EachTwoWeek,
    EachMonth,
    EachYear,
}

/// A `Todo`.
///
/// The required fields to create a `Todo`:
///
/// * `description`
///
/// The optional fields to create a `Todo`:
///
/// * `notice`
/// * `pin`
/// * `completed`
/// * `completed_at`
/// * `repeat_type`
/// * `remind_time`
/// * `estimated_pomo_count`
/// * `costed_pomo_count`
///
/// Not allowed fields in creating a pomo:
///
/// * `uuid`
/// * `created_at`
/// * `updated_at`
/// * `sub_todos`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<RepeatType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_pomo_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub costed_pomo_count: Option<u64>,

    #[serde(skip_serializing)]
    pub sub_todos: Option<Vec<Uuid>>,
}

/// A builder to construct the properties of a [`Todo`](struct.Todo.html).
#[derive(Debug)]
pub struct TodoBuilder {
    todo: Todo,
}

/// The parameters used in getting [`Todo`](struct.Todo.html)s.
#[derive(Debug)]
pub struct TodoParameter {
    completed: Option<bool>,
    completed_later_than: Option<DateTime<Utc>>,
    completed_earlier_than: Option<DateTime<Utc>>,
}

/// A `SubTodo`.
///
/// The required fields to create a `SubTodo`:
///
/// * `description`
///
/// The optional fields to create a `SubTodo`:
///
/// * `parent_uuid` (allowed, but will be dropped)
/// * `completed`
/// * `completed_at`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubTodo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

/// A builder to construct the properties of a [`SubTodo`](struct.SubTodo.html).
#[derive(Debug)]
pub struct SubTodoBuilder {
    sub_todo: SubTodo,
}

impl Default for Todo {
    fn default() -> Todo {
        Todo {
            uuid: None,
            created_at: None,
            updated_at: None,
            description: "New Todo Item via Rust client".to_owned(),
            notice: None,
            pin: None,
            completed: None,
            completed_at: None,
            repeat_type: None,
            remind_time: None,
            estimated_pomo_count: None,
            costed_pomo_count: None,
            sub_todos: None,
        }
    }
}

impl Default for TodoParameter {
    fn default() -> TodoParameter {
        TodoParameter {
            completed: Some(false),
            completed_later_than: None,
            completed_earlier_than: None,
        }
    }
}

impl Default for SubTodo {
    fn default() -> SubTodo {
        SubTodo {
            uuid: None,
            parent_uuid: None,
            created_at: None,
            updated_at: None,
            description: "New SubTodo Item via Rust client".to_owned(),
            completed: None,
            completed_at: None,
        }
    }
}

impl Todo {
    /// Creates an `TodoBuilder` to configure a [`Todo`](struct.Todo.html).
    pub fn builder() -> TodoBuilder {
        TodoBuilder { todo: Todo::default() }
    }
}

impl TodoBuilder {
    /// Set the `description` property.
    pub fn description<S: Into<String>>(&mut self, desc: S) -> &mut TodoBuilder {
        self.todo.description = desc.into();
        self
    }

    /// Build a [`Todo`](struct.Todo.html).
    pub fn finish(self) -> Todo {
        self.todo
    }
}

impl TodoParameter {
    /// Set the `completed` parameter.
    pub fn with_completed(&mut self, completed: bool) -> &mut TodoParameter {
        self.completed = Some(completed);
        self
    }

    /// Set the `completed_later_than` parameter.
    pub fn with_completed_later(&mut self, than: DateTime<Utc>) -> &mut TodoParameter {
        self.completed_later_than = Some(than);
        self
    }

    /// Set the `completed_earlier_than` parameter.
    pub fn with_completed_earlier(&mut self, than: DateTime<Utc>) -> &mut TodoParameter {
        self.completed_earlier_than = Some(than);
        self
    }

    /// Convert [`TodoParameter`](struct.TodoParameter.html) to query string.
    pub fn to_query(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        if let Some(completed) = self.completed {
            params.push(format!("completed={}", completed));
        }
        if let Some(completed_later_than) = self.completed_later_than {
            params.push(format!("completed_later_than={}", completed_later_than));
        }
        if let Some(completed_earlier_than) = self.completed_earlier_than {
            params.push(format!("completed_earlier_than={}", completed_earlier_than));
        }

        params.join("&")
    }
}

impl SubTodo {
    /// Creates an [`SubTodoBuilder`](struct.SubTodoBuilder.html)
    /// to configure a [`SubTodo`](struct.SubTodo.html).
    pub fn builder() -> SubTodoBuilder {
        SubTodoBuilder { sub_todo: SubTodo::default() }
    }
}

impl SubTodoBuilder {
    /// Set the `description` property.
    pub fn description<S: Into<String>>(&mut self, desc: S) -> &mut SubTodoBuilder {
        self.sub_todo.description = desc.into();
        self
    }

    /// Build a [`SubTodo`](struct.SubTodo.html).
    pub fn finish(self) -> SubTodo {
        self.sub_todo
    }
}

impl ::std::str::FromStr for RepeatType {
    type Err = ::std::io::Error;
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none"          => Ok(RepeatType::None),
            "each_day"      => Ok(RepeatType::EachDay),
            "each_week"     => Ok(RepeatType::EachWeek),
            "each_two_week" => Ok(RepeatType::EachTwoWeek),
            "each_month"    => Ok(RepeatType::EachMonth),
            "each_year"     => Ok(RepeatType::EachYear),
            _ => Err(::std::io::Error::new(
                ::std::io::ErrorKind::InvalidData,
                "invalid repeat type",
            )),
        }
    }
}

impl ::std::fmt::Display for RepeatType {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RepeatType::None        => write!(f, "none"),
            RepeatType::EachDay     => write!(f, "each_day"),
            RepeatType::EachWeek    => write!(f, "each_week"),
            RepeatType::EachTwoWeek => write!(f, "each_two_week"),
            RepeatType::EachMonth   => write!(f, "each_month"),
            RepeatType::EachYear    => write!(f, "each_year"),
        }
    }
}

impl ::std::fmt::Display for Todo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use serde_json::to_string_pretty;
        write!(f, "{}", to_string_pretty(self).unwrap_or_default())
    }
}

impl ::std::fmt::Display for SubTodo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use serde_json::to_string_pretty;
        write!(f, "{}", to_string_pretty(self).unwrap_or_default())
    }
}
