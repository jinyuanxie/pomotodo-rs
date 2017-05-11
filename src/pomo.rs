//Copyright 2017 KAMYUEN
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

extern crate serde_json;

use std::fmt;
use std::default::Default;

use uuid::Uuid;
use chrono::prelude::*;

use ::ShouldSkip;

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct Pomo {
    /// The unique id of the `Pomo`.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub uuid: Option<Uuid>,
    /// The create time of the `Pomo`.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub created_at: Option<DateTime<UTC>>,
    /// The update time of the `Pomo`.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub updated_at: Option<DateTime<UTC>>,
    /// The description of the `Pomo`.
    pub description: String,
    /// The started time of the `Pomo`.
    pub started_at: DateTime<UTC>,
    /// The ended time of the `Pomo`.
    pub ended_at: DateTime<UTC>,
    /// The local started time of the `Pomo`.
    /// Notice: **Stored in UTC format.**
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub local_started_at: Option<DateTime<UTC>>,
    /// The local ended time of the `Pomo`.
    /// Notice: **Store in UTC format.**
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub local_ended_at: Option<DateTime<UTC>>,
    /// The duration(in second) of the `Pomo`.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub length: Option<u64>,
    /// Whether this `Pomo` was abandoned.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub abandoned: Option<bool>,
    /// Whether this `Pomo` was created manually.
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub manual: Option<bool>,
}

impl Default for Pomo {
    /// The default value of a new `Pomo`.
    ///
    /// **Required fields:**
    ///
    /// - `description`
    /// - `started_at`
    /// - `length` or `ended_at`
    ///
    /// The result of a POST request with _empty body_
    ///
    /// ``` json
    /// {
    ///   "code": "InvalidContent",
    ///   "message": "Validation failed",
    ///   "description": "Validation failed",
    ///   "errors": [
    ///    {
    ///       "path": "started_at",
    ///       "type": "missing_field",
    ///       "message": "\"started_at\" is required"
    ///    },
    ///    {
    ///       "path": "description",
    ///       "type": "missing_field",
    ///       "message": "\"description\" is required"
    ///     },
    ///     {
    ///         "path": "value",
    ///         "type": "invalid",
    ///         "message": "\"value\" must contain at least one of [length, ended_at]"
    ///     }
    ///    ],
    ///    "documentation_url": "https://pomotodo.com/developer"
    /// }
    /// ```
    ///
    /// Read more: [CreatePomo](https://pomotodo.github.io/api-doc/#api-Pomo-CreatePomo)
    fn default() -> Pomo {
        Pomo {
            uuid: None,
            created_at: None,
            updated_at: None,
            description: "New Pomo".to_string(),
            started_at: UTC::now(),
            ended_at: UTC::now(),
            local_started_at: None,
            local_ended_at: None,
            length: None,
            abandoned: Some(false),
            manual: Some(true),
            // For using the api, this field must be true.
        }
    }
}

impl fmt::Display for Pomo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               serde_json::to_string_pretty(self).unwrap_or_default())
    }
}
