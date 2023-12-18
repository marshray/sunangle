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

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TimecodeKind {
    frame_rate: FrameRate,
    drop_smpte: bool,
}

impl TimecodeKind {
    pub fn new(frame_rate: &FrameRate) -> TimecodeKind {
        TimecodeKind {
            frame_rate: *frame_rate,
            drop_smpte: false,
        }
    }

    #[cfg(test)]
    #[rustfmt::skip]
    pub fn try_new(frame_rate: FrameRate, drop_smpte: bool) -> Option<TimecodeKind> {
        match (drop_smpte, frame_rate.frames(), frame_rate.seconds()) {
              (false,     _,    _)
            | ( true, 30000, 1001)
            | ( true, 60000, 1001) => Some(TimecodeKind { frame_rate, drop_smpte }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Timecode {
    kind: TimecodeKind,
    hh: u32,
    mm: u8,  // 0 - 59
    ss: u8,  // 0 - 59
    ff: u16, // 0 < kind.frame_rate.fps_ceil()
}

//? TODO: Timecode Display
//? TODO: Timecode Debug
//? TODO: Timecode Parse, from string, etc

impl Timecode {
    pub fn new(kind: &TimecodeKind) -> Timecode {
        Timecode {
            kind: *kind,
            hh: 0,
            mm: 0,
            ss: 0,
            ff: 0,
        }
    }

    pub fn kind(&self) -> &TimecodeKind {
        &self.kind
    }
}

pub enum SmpteMaybeDroppedTimecode {
    Dropped(Timecode),
    Framed(Timecode),
}

impl Timecode {
    pub fn next_including_dropped(mut self) -> SmpteMaybeDroppedTimecode {        
        use SmpteMaybeDroppedTimecode::*;

        assert!(*self.kind.frame_rate.ratio().denom() == 1, "TODO support dropped timecode properly");
        assert!(!self.kind.drop_smpte, "TODO properly support dropped timecode");

        let fps = self.kind.frame_rate.fps_floor();
        
        self.ff += 1;
        if fps <= self.ff {
            self.ff = 0;
            self.ss += 1;

            if self.ss == 60 {
                self.ss = 0;
                self.mm += 1;

                if self.mm == 60 {
                    self.mm = 0;
                    self.hh += 1;
                }
            }
        }

        Framed(self)
    }

    pub fn next(mut self) -> Timecode {
        use SmpteMaybeDroppedTimecode::*;

        let mut self_ = self;
        loop {
            match self_.next_including_dropped() {
                Dropped(tc) => {
                    self_ = tc;
                }
                Framed(tc) => {
                    return tc;
                }
            }
        }
    }
}
