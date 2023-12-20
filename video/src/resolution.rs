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
#![allow(non_upper_case_globals)]
#![allow(clippy::new_without_default)] //? TODO for development
#![allow(clippy::let_and_return)] //? TODO for development
#![allow(clippy::redundant_closure)] //? TODO for development
                                     //#![allow(clippy::too_many_arguments)]

//? use std::any::Any;
use std::borrow::Cow;
use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
use num_rational::Ratio;
use num_traits::identities::Zero;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct VideoResolution {
    /// Name.
    pub names: Vec<Cow<'static, str>>,

    /// Pixel width, height.
    pub dims_px: [u16; 2],

    /// Aspect ratio of an individual pixel.
    pub pixel_aspect: Ratio<u16>,
}

impl VideoResolution {
    pub fn phys_aspect_px(&self) -> Ratio<u32> {
        let n = u32::from(self.dims_px[0]) * u32::from(*self.pixel_aspect.numer());
        if n == 0 {
            Ratio::zero()
        } else {
            let d = u32::from(self.dims_px[1]) * u32::from(*self.pixel_aspect.denom());
            if d == 0 {
                Ratio::from(u32::MAX)
            } else {
                Ratio::new(n, d)
            }
        }
    }
}

pub static VIDEO_RESOLUTION_720: Lazy<VideoResolution> = Lazy::new(|| VideoResolution {
    names: ["720", "1280x720", "HD", "WXGA"]
        .iter()
        .map(|&s| s.into())
        .collect(),
    dims_px: [1280, 720],
    pixel_aspect: Ratio::new_raw(1, 1),
});

pub static VIDEO_RESOLUTION_1080: Lazy<VideoResolution> = Lazy::new(|| VideoResolution {
    names: ["1080", "1920x1080", "Full HD"]
        .iter()
        .map(|&s| s.into())
        .collect(),
    dims_px: [1920, 1080],
    pixel_aspect: Ratio::new_raw(1, 1),
});

pub static COMMMON_VIDEO_RESOLUTIONS: Lazy<Vec<&VideoResolution>> =
    Lazy::new(|| vec![&VIDEO_RESOLUTION_720, &VIDEO_RESOLUTION_1080]);

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn t() -> anyhow::Result<()> {
        assert_ron_snapshot!(*VIDEO_RESOLUTION_720, @"");
        assert_ron_snapshot!(*VIDEO_RESOLUTION_1080, @"");
        Ok(())
    }
}
