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
use std::fmt::{Debug, DebugStruct, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::*;

/// Information about a Dimension of a [`CoordinateSystem`].
#[derive(Debug, PartialEq, PartialOrd, Serialize)]
pub struct DimensionInfo {
    /// The conventional name for a variable representing a coordinate of this dimension. E.g. "x".
    /// A single letter is preferred.
    pub name: Cow<'static, str>,

    /// Inclusive min and exclusive max.
    /// For example, an angular dimension expressed in radians may have bounds `(0.0, std::f64::consts::TAU)`
    pub opt_cyclic_bounds: Option<(f64, f64)>,
}

impl Display for DimensionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
