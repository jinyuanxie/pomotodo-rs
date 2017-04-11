extern crate serde_json;

use std::fmt;
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    username: String,
    email: String,
    timezone: String,
    register_time: DateTime<UTC>,
    pro_expires_time: DateTime<UTC>,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               serde_json::to_string_pretty(self).unwrap_or_default())
    }
}
