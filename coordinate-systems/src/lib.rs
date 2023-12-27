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

//! This crate is intended to be useful for anything involving space- or time-axis coordinate
//! systems.
//!
//! Some parts are inspired by formal specifications for GIS data, and may even reference them,
//! but does not attempt conformance to any documented standards.
//!
//! For example,
//! - Open Geospatial Consortium
//!   - [Well-known text representation of coordinate reference systems](https://www.ogc.org/standard/wkt-crs/)
//!     - [18-010r11 v2.1.11](https://docs.ogc.org/is/18-010r11/18-010r11.pdf)
//!     - [18-010r7 v2.0.6](https://docs.ogc.org/is/18-010r7/18-010r7.html)
//!
//! However, it is not an OO design. The types in this crate are intended to be used with an
//! entity component system (ECS).

//? use std::any::Any;
use std::borrow::Cow;
//? use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumProperty, EnumString};

type CowStaticStr = Cow<'static, str>;

mod core;
#[cfg(test)]
mod core_test;

//mod cs;
//mod geom; // Ellipsoid3Sphere, Ellipsoid3Oblate, Ellipsoid3Triaxial
//mod datum;
//mod crs;
