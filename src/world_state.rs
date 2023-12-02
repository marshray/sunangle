// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(dead_code)] //? TODO for development
#![allow(unused_mut)] //? TODO for development
#![allow(unused_variables)] //? TODO for development
#![allow(unused_imports)] //? TODO for development
#![allow(non_snake_case)] //? TODO for development
#![allow(clippy::new_without_default)] //? TODO for development
#![allow(clippy::too_many_arguments)]

//? use use std::fmt::Display;
//? use std::ops::RangeInclusive;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDate, Utc};

use crate::tai::DateTimeTai;

pub struct WorldState {
    tai: DateTimeTai,
}

impl WorldState {
    pub fn default_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2000, 3, 1).unwrap()
    }

    pub fn default_utc() -> DateTime<chrono::Utc> {
        Self::default_date()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
    }

    pub fn default_tai() -> DateTimeTai {
        Self::default_utc().into()
    }

    pub fn new(opt_tai: Option<DateTimeTai>) -> WorldState {
        let tai = opt_tai.unwrap_or_else(Self::default_tai);
        Self::world_at_tai(tai)
    }

    pub fn world_at_tai(tai: DateTimeTai) -> WorldState {
        WorldState { tai }
    }
}