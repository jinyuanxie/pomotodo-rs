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

use chrono::prelude::*;

/// Account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    username: String,
    email: String,
    timezone: String,
    register_time: DateTime<Utc>,
    pro_expires_time: DateTime<Utc>,
}

impl ::std::fmt::Display for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use serde_json::to_string_pretty;
        write!(f, "{}", to_string_pretty(self).unwrap_or_default())
    }
}
