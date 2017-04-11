//! The todo module.
extern crate serde_json;

use std::io;
use std::fmt;
use std::str::FromStr;
use std::default::Default;

use uuid::Uuid;
use chrono::prelude::*;

use ::ShouldSkip;

/// The repeat type of the `Todo` task.
#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="snake_case")]
pub enum RepeatType {
    None,
    EachDay,
    EachWeek,
    EachTwoWeek,
    EachMonth,
    EachYear,
}

impl FromStr for RepeatType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(RepeatType::None),
            "each_day" => Ok(RepeatType::EachDay),
            "each_week" => Ok(RepeatType::EachWeek),
            "each_two_week" => Ok(RepeatType::EachTwoWeek),
            "each_month" => Ok(RepeatType::EachMonth),
            "each_year" => Ok(RepeatType::EachYear),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid repeat type.")),
        }
    }
}

impl fmt::Display for RepeatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RepeatType::None => write!(f, "none"),
            RepeatType::EachDay => write!(f, "each_day"),
            RepeatType::EachWeek => write!(f, "each_week"),
            RepeatType::EachTwoWeek => write!(f, "each_two_week"),
            RepeatType::EachMonth => write!(f, "each_month"),
            RepeatType::EachYear => write!(f, "each_year"),
        }
    }
}

/// A `Todo` task.
#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    /// The unique id of the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub uuid: Option<Uuid>,
    /// The create time of the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub created_at: Option<DateTime<UTC>>,
    /// The update time of the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub updated_at: Option<DateTime<UTC>>,
    /// The description of the task.
    pub description: String,
    /// Whether notice the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub notice: Option<String>,
    /// Whether pin the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub pin: Option<bool>,
    /// Whether the task was completed.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub completed: Option<bool>,
    /// The completed time of the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub completed_at: Option<DateTime<UTC>>,
    /// The `RepeatType` of the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub repeat_type: Option<RepeatType>,
    /// When remind the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub remind_time: Option<DateTime<UTC>>,
    /// How many `Pomo` to finish this task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub estimated_pomo_count: Option<u64>,
    /// The costed `Pomo` in doing this task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub costed_pomo_count: Option<u64>,
    /// The sub task of this task.
    #[serde(skip_serializing)]
    pub sub_todos: Option<Vec<Uuid>>,
}

impl Default for Todo {
    /// The default value of a new `Todo` task.
    ///
    /// **Required fields:**
    ///
    /// - `description`
    ///
    /// The result of a POST request with _empty body_
    ///
    /// ``` json
    /// {
    ///    "code": "InvalidContent",
    ///    "message": "Validation failed",
    ///    "description": "Validation failed",
    ///    "errors": [
    ///     {
    ///        "path": "description",
    ///        "type": "missing_field",
    ///        "message": "\"description\" is required"
    ///     }
    ///    ],
    ///    "documentation_url": "https://pomotodo.com/developer"
    /// }
    /// ```
    ///
    /// Read more: [CreateTodo](https://pomotodo.github.io/api-doc/#api-Todo-CreateTodos)
    fn default() -> Todo {
        Todo {
            uuid: None,
            created_at: None,
            updated_at: None,
            description: "New Todo Item".to_string(),
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

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

/// A `SubTodo` task.
#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct SubTodo {
    /// The unique id of the sub task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub uuid: Option<Uuid>,
    /// The unique id of the parent task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub parent_uuid: Option<Uuid>,
    /// The create time of the sub task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub created_at: Option<DateTime<UTC>>,
    /// The update time of the sub task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub updated_at: Option<DateTime<UTC>>,
    /// The description of the sub task
    pub description: String,
    /// Whether the task was completed.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub completed: Option<bool>,
    /// The completed time of the task.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub completed_at: Option<DateTime<UTC>>,
}

impl Default for SubTodo {
    /// The default value of a new `SubTodo` task.
    ///
    /// **Required fields:**
    ///
    /// - `description`
    ///
    /// The result of a POST request with _empty body_
    ///
    /// ``` json
    /// {
    ///    "code": "InvalidContent",
    ///    "message": "Validation failed",
    ///    "description": "Validation failed",
    ///    "errors": [
    ///     {
    ///        "path": "description",
    ///        "type": "missing_field",
    ///        "message": "\"description\" is required"
    ///     }
    ///    ],
    ///    "documentation_url": "https://pomotodo.com/developer"
    /// }
    /// ```
    ///
    /// Read more: [CreateTodo](https://pomotodo.github.io/api-doc/#api-Todo-CreateTodos)
    fn default() -> SubTodo {
        SubTodo {
            uuid: None,
            parent_uuid: None,
            created_at: None,
            updated_at: None,
            description: "New SubTodo Item".to_string(),
            completed: None,
            completed_at: None,
        }
    }
}

impl fmt::Display for SubTodo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               serde_json::to_string_pretty(self).unwrap_or_default())
    }
}
