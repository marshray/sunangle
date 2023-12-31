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
#![allow(clippy::let_and_return)] //? TODO for development
#![allow(clippy::redundant_closure)] //? TODO for development
#![allow(clippy::too_many_arguments)]

//? use std::any::Any;
//? use std::borrow::Cow;
//? use std::fmt::Display;
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
//? use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::tai::DateTimeTai;

//======================================================================== time

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct TimeState {
    pub tai: DateTimeTai,
}

impl Default for TimeState {
    fn default() -> Self {
        TimeState {
            tai: DateTimeTai::EPOCH_400,
        }
    }
}

impl TimeState {
    pub fn new(opt_tai: Option<DateTimeTai>) -> Self {
        let mut self_ = Self::default();
        if let Some(tai) = opt_tai {
            self_.tai = tai
        }
        self_
    }
}

//======================================================================== observer position

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct ObserverPositionState {
    //?
}

//======================================================================== overall world state

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct WorldState {
    pub time: TimeState,
    pub observer_position: ObserverPositionState,
}

impl WorldState {
    pub fn new(
        opt_time_state: Option<TimeState>,
        opt_observer_position_state: Option<ObserverPositionState>,
    ) -> WorldState {
        let time = opt_time_state.unwrap_or_else(|| TimeState::new(None));
        let observer_position =
            opt_observer_position_state.unwrap_or_else(|| ObserverPositionState::default());
        WorldState {
            time,
            observer_position,
        }
    }
}

//======================================================================== world model

pub struct WorldModel {
    world_state: WorldState,
    // Positions of everything
}

impl WorldModel {
    pub fn new(world_state: WorldState) -> WorldModel {
        WorldModel { world_state }
    }
}
