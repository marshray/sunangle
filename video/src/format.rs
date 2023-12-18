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
use std::borrow::Cow;
//? use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
pub struct VideoFormat {
    /// Name.
    pub name: Cow<'static, str>,

    /// Resolution.
    pub resolution: VideoResolution,
    
    /// Frame rate.
    pub frame_rate: FrameRate,
}
/*
pub const VIDEO_FORMAT_1080p30: VideoFormat = VideoFormat { name: "1080p30".into(), resolution: VIDEO_RESOLUTION_1080p, frame_rate: FrameRate { num: 30, den: 1 } };

pub const COMMMON_VIDEO_FORMATS: [VideoFormat] = [
    VIDEO_FORMAT_1080p30,
    VideoFormat { name: "1080p60".into(), resolution: VIDEO_RESOLUTION_1080p, frame_rate: FrameRate { num: 60, den: 1 } },
];
*/
//? TODO: tests
