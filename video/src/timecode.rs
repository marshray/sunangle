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
use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Timecode {
    kind: TimecodeKind,
    hh: u32,
    mm: u8,  // 0 - 59
    ss: u8,  // 0 - 59
    ff: u16, // 0 < kind.framerate.fps_ceil()
}

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

impl Display for Timecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}:{:02}", self.hh, self.mm, self.ss, self.ff)
    }
}

impl Debug for Timecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: ", self.kind)?;
        Display::fmt(self, f)
    }
}

pub enum SmpteMaybeDroppedTimecode {
    Dropped(Timecode),
    Framed(Timecode),
}

impl Timecode {
    pub fn next_including_dropped(mut self) -> SmpteMaybeDroppedTimecode {
        use SmpteMaybeDroppedTimecode::*;

        assert!(
            *self.kind.framerate().ratio().denom() == 1,
            "TODO support dropped timecode properly"
        );
        assert!(
            !self.kind.drop_smpte(),
            "TODO properly support dropped timecode"
        );

        let fps = self.kind.framerate().fps_floor();

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

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn t() -> anyhow::Result<()> {
        let mut tc = Timecode::new(&TIMECODEKIND_15_FPS);
        assert_ron_snapshot!(tc, @r###"
        Timecode(
          kind: TimecodeKind(
            framerate: FrameRate((15, 1), (15, 1)),
            drop_smpte: false,
          ),
          hh: 0,
          mm: 0,
          ss: 0,
          ff: 0,
        )
        "###);
        assert_ron_snapshot!(tc.to_string(), @r###""00:00:00:00""###);
        
        tc = tc.next();
        assert_ron_snapshot!(tc.to_string(), @r###""00:00:00:01""###);
        for _ in 0..13 { tc = tc.next(); }
        assert_ron_snapshot!(tc.to_string(), @r###""00:00:00:14""###);
        tc = tc.next();
        assert_ron_snapshot!(tc.to_string(), @r###""00:00:01:00""###);

        for _ in 0..(58*15 + 14) { tc = tc.next(); }
        assert_ron_snapshot!(tc.to_string(), @r###""00:00:59:14""###);
        tc = tc.next();
        assert_ron_snapshot!(tc.to_string(), @r###""00:01:00:00""###);

        for _ in 0..(58*60*15 + 59*15 + 14) { tc = tc.next(); }
        assert_ron_snapshot!(tc.to_string(), @r###""00:59:59:14""###);
        tc = tc.next();
        assert_ron_snapshot!(tc.to_string(), @r###""01:00:00:00""###);

        for _ in 0..(59*60*15 + 59*15 + 14) { tc = tc.next(); }
        assert_ron_snapshot!(tc.to_string(), @r###""01:59:59:14""###);
        tc = tc.next();
        assert_ron_snapshot!(tc.to_string(), @r###""02:00:00:00""###);

        Ok(())
    }
}
