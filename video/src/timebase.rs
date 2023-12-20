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
//? use log::{debug, error, info, trace, warn};
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::*;

/* #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
pub struct TimebaseSpec {
    timecode_kind: TimecodeKind,
}

impl TimebaseSpec {
    pub fn new(timecode_kind: &TimecodeKind) -> TimebaseSpec {
        TimebaseSpec {
            timecode_kind,
        }
    }
} */

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Timebase {
    next_timecode: Timecode,
    //timebase_spec: TimebaseSpec,
}

impl Timebase {
    pub fn new(timecode_kind: &TimecodeKind) -> Timebase {
        Timebase {
            next_timecode: Timecode::new(timecode_kind),
        }
    }

    pub fn timecode_kind(&self) -> &TimecodeKind {
        self.next_timecode.kind()
    }

    pub fn set_next(&mut self, timecode: &Timecode) -> Result<()> {
        ensure!(
            self.timecode_kind() == timecode.kind(),
            "TimecodeKind mismatch"
        );
        self.next_timecode = *timecode;
        Ok(())
    }

    pub fn take_next_timecode(&mut self) -> Timecode {
        let tc = self.next_timecode;
        self.next_timecode.next();
        tc
    }
}
