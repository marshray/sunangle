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
//? use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

use anyhow::{anyhow, bail, ensure, Context, Result};
//? use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use crate::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TimecodeKind {
    framerate: FrameRate,
    drop_smpte: bool,
}

impl TimecodeKind {
    /// Constructs a new [`TimecodeKind`] for a given [`FrameRate`].
    pub const fn new(framerate: &FrameRate) -> TimecodeKind {
        TimecodeKind {
            framerate: *framerate,
            drop_smpte: false,
        }
    }

    /// Attempts to construct a new [`TimecodeKind`] for a given [`FrameRate`] and `drop_smpte` value.
    #[rustfmt::skip]
    pub fn try_new(framerate: FrameRate, drop_smpte: bool) -> Result<TimecodeKind> {
        match (drop_smpte, framerate.frames(), framerate.seconds()) {
              (false,     _,    _)
            | ( true, 30000, 1001)
            | ( true, 60000, 1001) => Ok(TimecodeKind { framerate, drop_smpte }),
            _ => Err(anyhow!("Drop SMPTE is only supported for (30000 and 60000)/1001 frame rates")),
        }
    }

    /// The [`FrameRate`].
    pub fn framerate(&self) -> FrameRate {
        self.framerate
    }

    /// Indicates whether this is a SMPTE "drop frame" (more correctly "drop timecode") kind of timecode.
    pub fn drop_smpte(&self) -> bool {
        self.drop_smpte
    }
}

pub static TIMECODEKIND_15_FPS: Lazy<TimecodeKind> =
    Lazy::new(|| TimecodeKind::new(&FRAMERATE_15_FPS));
pub static TIMECODEKIND_30_FPS: Lazy<TimecodeKind> =
    Lazy::new(|| TimecodeKind::new(&FRAMERATE_30_FPS));
pub static TIMECODEKIND_60_FPS: Lazy<TimecodeKind> =
    Lazy::new(|| TimecodeKind::new(&FRAMERATE_60_FPS));

#[rustfmt::skip]
pub static COMMMON_TIMECODEKINDS: Lazy<Vec<&TimecodeKind>> = Lazy::new(|| vec![
    &*TIMECODEKIND_15_FPS,
    &*TIMECODEKIND_30_FPS,
    &*TIMECODEKIND_60_FPS,
]);
