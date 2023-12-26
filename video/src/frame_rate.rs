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

use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
use num_integer::Integer;
use num_rational::Ratio;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// Frames per some number of seconds.
///
/// The [`Ratio`] type likes to reduce fractions, but maybe there could theoretically be cases
/// where the un-reduced fraction could be significant.
///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct FrameRate([u16; 2], Ratio<u16>);

impl FrameRate {
    /// Attempts to create a new [`FrameRate`] for an integer frames-per-second.
    pub fn try_new_fps(fps: u16) -> Result<Self> {
        Self::try_new(fps, 1)
    }

    /// Attempts to create a new [`FrameRate`].
    /// Both parameters accept any numbers except for `0`.
    pub fn try_new(frames: u16, seconds: u16) -> Result<Self> {
        if frames == 0 || seconds == 0 {
            Err(anyhow!("Frames and seconds must be nonzero."))
        } else {
            let pr = [frames, seconds];
            let r = Into::<(u16, u16)>::into(pr).into(); // reduces
            Ok(FrameRate(pr, r))
        }
    }

    /// The not-reduced numerator.
    pub const fn frames(&self) -> u16 {
        self.0[0]
    }

    /// The not-reduced denominator.
    pub const fn seconds(&self) -> u16 {
        self.0[1]
    }

    /// The ratio, possibly reduced.
    pub const fn ratio(&self) -> Ratio<u16> {
        self.1
    }

    /// floor(frames per second)
    pub const fn fps_floor(&self) -> u16 {
        self.0[0] / self.0[1]
    }

    /// ceil(frames per second)
    pub fn fps_ceil(&self) -> u16 {
        // Ratio::ceil() doesn't catch overflow, so we have to roll our own.
        match self.0[0].div_mod_floor(&self.0[1]) {
            (quo, 0) => quo,
            (quo, _) => quo.saturating_add(1),
        }
    }
}

pub static FRAMERATE_15_FPS: Lazy<FrameRate> = Lazy::new(|| FrameRate::try_new_fps(15).unwrap());
pub static FRAMERATE_29_97_FPS: Lazy<FrameRate> =
    Lazy::new(|| FrameRate::try_new(30000, 1001).unwrap());
pub static FRAMERATE_30_FPS: Lazy<FrameRate> = Lazy::new(|| FrameRate::try_new_fps(30).unwrap());
pub static FRAMERATE_59_94_FPS: Lazy<FrameRate> =
    Lazy::new(|| FrameRate::try_new(60000, 1001).unwrap());
pub static FRAMERATE_60_FPS: Lazy<FrameRate> = Lazy::new(|| FrameRate::try_new_fps(60).unwrap());

#[rustfmt::skip]
pub static COMMMON_FRAMERATES: Lazy<Vec<&FrameRate>> = Lazy::new(|| vec![
    &FRAMERATE_15_FPS,
    &FRAMERATE_30_FPS,
    &FRAMERATE_60_FPS,
]);

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use anyhow::{anyhow, bail, ensure, Context, Result};
    use insta::assert_ron_snapshot;

    #[test]
    fn t() {
        assert_ron_snapshot!(*COMMMON_FRAMERATES, @r###"
        [
          FrameRate((15, 1), (15, 1)),
          FrameRate((30, 1), (30, 1)),
          FrameRate((60, 1), (60, 1)),
        ]
        "###);
    }

    #[test]
    fn t2997() -> anyhow::Result<()> {
        let fr = *FRAMERATE_29_97_FPS;
        assert_ron_snapshot!(fr.frames(), @"30000");
        assert_ron_snapshot!(fr.seconds(), @"1001");
        assert_ron_snapshot!(fr.ratio(), @"(30000, 1001)");
        assert_ron_snapshot!(fr.fps_floor(), @"29");
        assert_ron_snapshot!(fr.fps_ceil(), @"30");
        Ok(())
    }

    #[test]
    fn t5994() -> anyhow::Result<()> {
        let fr = *FRAMERATE_59_94_FPS;
        assert_ron_snapshot!(fr.frames(), @"60000");
        assert_ron_snapshot!(fr.seconds(), @"1001");
        assert_ron_snapshot!(fr.ratio(), @"(60000, 1001)");
        assert_ron_snapshot!(fr.fps_floor(), @"59");
        assert_ron_snapshot!(fr.fps_ceil(), @"60");
        Ok(())
    }

    #[test]
    fn t_reducible() -> anyhow::Result<()> {
        let fr = FrameRate::try_new(60, 2)?;
        assert_ron_snapshot!(fr.frames(), @"60");
        assert_ron_snapshot!(fr.seconds(), @"2");
        assert_ron_snapshot!(fr.ratio(), @"(30, 1)");
        assert_ron_snapshot!(fr.fps_floor(), @"30");
        assert_ron_snapshot!(fr.fps_ceil(), @"30");
        Ok(())
    }
}
