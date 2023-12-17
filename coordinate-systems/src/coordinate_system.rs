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

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use serde::{Deserialize, Serialize};

use crate::*;

pub trait CoordinateSystem: Debug + Display {
    /// Returns the name of the coordinate system.
    fn name(&self) -> Cow<'_, str>;

    /// Returns `true` iff the coordinate system is a [Cartesian coordinate system](https://en.wikipedia.org/wiki/Cartesian_coordinate_system)
    /// of (positive) `D` dimension. I.e., there are `D` orthogonal axes.
    ///
    /// Perhaps it could implement [`cgmath::EuclideanSpace`](https://docs.rs/cgmath/latest/cgmath/trait.EuclideanSpace.html) trait.
    fn cartesian(&self) -> bool;

    /// Returns the number of dimensions, i.e., coordinates needed to uniquely identify a point.
    fn cnt_dimensions(&self) -> usize;

    /// Returns information about the dimension by 0-based index, or None if ix >= the number of dimensions.
    fn dimension_info(&self, ix: usize) -> Option<&DimensionInfo>;
}

pub trait CoordinateSystemD<const D: usize>: CoordinateSystem {
    /// The D-dimensional array type needed to represent a point.
    //type P = [u64; D];

    /// The number of dimensions.
    const D: usize = D;

    /// The D-dimensional array representing the coordinate system origin.
    //const ORIGIN: P = [0.0_f64; D];
    const ORIGIN: [f64; D] = [0.0_f64; D];

    fn origin(&self) -> [f64; D] {
        Self::ORIGIN
    }
}
