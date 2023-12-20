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
use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::tai::DateTimeTai;
use crate::ui::animation_ctrl_window;
use crate::world_state::TimeState;

//======================================================================== timebase and animation

/* #[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpochAndElapsed {
    epoch_tai: DateTimeTai,

    #[serde(skip)]
    opt_instant: Option<Instant>,
} */

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub enum AnimationTimebase {
    #[default]
    SystemClock,

    Fixed {
        tai: DateTimeTai,
    },

    EpochAndElapsed {
        epoch_tai: DateTimeTai,

        #[serde(skip)]
        opt_instant: Option<Instant>,
    },

    Video {
        timebase: video::Timebase,
    },
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub enum AnimationStateEn {
    Paused,
    #[default]
    Play,
    Rewind,
    FF,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct AnimationState {
    pub en: AnimationStateEn,

    /// log10 of the animation speed.
    pub log10_animation_speed: f64,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            en: Default::default(),
            log10_animation_speed: 0.0,
        }
    }
}

impl AnimationState {
    pub fn is_animating(&self) -> bool {
        !matches!(self.en, AnimationStateEn::Paused) && -1000.0 < self.log10_animation_speed
    }
}

//======================================================================== overall view state

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ViewState {
    time: TimeState,
}

impl ViewState {
    pub fn new(opt_time_state: Option<TimeState>) -> ViewState {
        let time = opt_time_state.unwrap_or_else(|| TimeState::new(None));
        Self { time }
    }
}
