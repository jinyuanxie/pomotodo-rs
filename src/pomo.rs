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

/// An `Pomo`.
///
/// The required fields to create a `Pomo`:
///
/// * `description`
/// * `started_at`
/// * `ended_at` or `length`
///
/// **Note:** If specify both `ended_at` and `length`, the `length`
/// will be droppd and recalculated.
///
/// Not allowed fields in creating a `Pomo`:
///
/// * `uuid`
/// * `created_at`
/// * `updated_at`
///
/// Others will be keep their values except the `manual`, it MUST be `true`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pomo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    pub description: String,

    pub started_at: DateTime<Utc>,

    pub ended_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_started_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ended_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
}

/// A builder to construct the properties of a [`Pomo`](struct.Pomo.html).
#[derive(Debug)]
pub struct PomoBuilder {
    pomo: Pomo,
}

/// The parameters used in getting [`Pomo`](struct.Pomo.html)s.
#[derive(Debug)]
pub struct PomoParameter {
    abandoned: Option<bool>,
    manual: Option<bool>,
    started_later_than: Option<DateTime<Utc>>,
    started_earlier_than: Option<DateTime<Utc>>,
    ended_later_than: Option<DateTime<Utc>>,
    ended_earlier_than: Option<DateTime<Utc>>,
}

impl Default for Pomo {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn default() -> Pomo {
        Pomo {
            uuid:             None,
            created_at:       None,
            updated_at:       None,
            description:      "New Pomo via Rust client".to_string(),
            started_at:       Utc::now(),
            ended_at:         Utc::now(),
            local_started_at: None,
            local_ended_at:   None,
            length:           None,
            abandoned:        Some(false),
            manual:           Some(true),
        }
    }
}

impl Default for PomoParameter {
    fn default() -> PomoParameter {
        PomoParameter {
            abandoned: Some(false),
            manual: Some(false),
            started_later_than: None,
            started_earlier_than: None,
            ended_later_than: None,
            ended_earlier_than: None,
        }
    }
}

impl Pomo {
    /// Creates an [`PomoBuilder`](struct.PomoBuilder.html)
    /// to configure a [`Pomo`](struct.Pomo.html).
    pub fn builder() -> PomoBuilder {
        PomoBuilder { pomo: Pomo::default() }
    }
}

impl PomoBuilder {
    /// Set the `started_at` property.
    pub fn started_at(&mut self, time: DateTime<Utc>) -> &mut PomoBuilder {
        self.pomo.started_at = time;
        self
    }

    /// Set the `end_at` property.
    pub fn ended_at(&mut self, time: DateTime<Utc>) -> &mut PomoBuilder {
        self.pomo.ended_at = time;
        self
    }

    /// Set the `description` property.
    pub fn description<T: Into<String>>(&mut self, desc: T) -> &mut PomoBuilder {
        self.pomo.description = desc.into();
        self
    }

    /// Build a [`Pomo`](struct.Pomo.html).
    pub fn finish(self) -> Pomo {
        self.pomo
    }
}

impl PomoParameter {
    /// Set the `abandoned` parameter.
    pub fn with_abandoned(&mut self, abandoned: bool) -> &mut PomoParameter {
        self.abandoned = Some(abandoned);
        self
    }

    /// Set the `manual` parameter.
    pub fn with_manual(&mut self, manual: bool) -> &mut PomoParameter {
        self.manual = Some(manual);
        self
    }

    /// Set the `started_later_than` parameter.
    pub fn with_started_later(&mut self, than: DateTime<Utc>) -> &mut PomoParameter {
        self.started_later_than = Some(than);
        self
    }

    /// Set the `started_earlier_than` parameter.
    pub fn with_started_earlier(&mut self, than: DateTime<Utc>) -> &mut PomoParameter {
        self.started_earlier_than = Some(than);
        self
    }

    /// Set the `ended_later_than` parameter.
    pub fn with_ended_later(&mut self, than: DateTime<Utc>) -> &mut PomoParameter {
        self.ended_later_than = Some(than);
        self
    }

    /// Set the `ended_earlier_than` parameter.
    pub fn with_ended_earlier(&mut self, than: DateTime<Utc>) -> &mut PomoParameter {
        self.ended_earlier_than = Some(than);
        self
    }

    /// Convert [`PomoParameter`](struct.PomoParameter.html) to query string.
    pub fn to_query(&self) -> String {
        let mut paras: Vec<String> = Vec::new();

        if let Some(abandoned) = self.abandoned {
            paras.push(format!("abandoned={}", abandoned));
        }
        if let Some(manual) = self.manual {
            paras.push(format!("manual={}", manual));
        }
        if let Some(started_later_than) = self.started_later_than {
            paras.push(format!("started_later_than={}", started_later_than));
        }
        if let Some(started_earlier_than) = self.started_earlier_than {
            paras.push(format!("started_earlier_than={}", started_earlier_than));
        }
        if let Some(ended_later_than) = self.ended_later_than {
            paras.push(format!("ended_later_than={}", ended_later_than));
        }
        if let Some(ended_earlier_than) = self.ended_earlier_than {
            paras.push(format!("ended_earlier_than={}", ended_earlier_than));
        }

        paras.join("&")
    }
}

impl ::std::fmt::Display for Pomo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use serde_json::to_string_pretty;
        write!(f, "{}", to_string_pretty(self).unwrap_or_default())
    }
}
