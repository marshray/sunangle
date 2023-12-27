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

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};

mod format;
pub use crate::format::VideoFormat;

mod frame_rate;
#[rustfmt::skip]
pub use crate::frame_rate::{FrameRate,
    FRAMERATE_15_FPS,
    FRAMERATE_30_FPS,
    FRAMERATE_60_FPS,
    COMMMON_FRAMERATES,    
};

mod resolution;
pub use crate::resolution::VideoResolution;

mod timebase;
pub use crate::timebase::Timebase;

mod timecode;
pub use crate::timecode::Timecode;

mod timecode_kind;
#[rustfmt::skip]
pub use crate::timecode_kind::{
    TimecodeKind,
    TIMECODEKIND_15_FPS,
    TIMECODEKIND_30_FPS,
    TIMECODEKIND_60_FPS,
    COMMMON_TIMECODEKINDS,
};
