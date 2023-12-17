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
use std::ops::Deref;
//? use std::fmt::Display;
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};

mod dimension_info;
pub use crate::dimension_info::DimensionInfo;

mod coordinate_system;
pub use crate::coordinate_system::{CoordinateSystem, CoordinateSystemD};

mod cartesian_coordinate_system;
pub use crate::cartesian_coordinate_system::{
    CartesianCoordinateSystem, CartesianCoordinateSystemD,
};

mod specific_coordinate_system;
pub use crate::specific_coordinate_system::SpecificCoordinateSystem;

mod specific_cartesian_coordinate_system;
pub use crate::specific_cartesian_coordinate_system::SpecificCartesianCoordinateSystem;

mod generic_systems;
pub use crate::generic_systems::{XY, XYZ};

pub static COORDINATE_SYSTEMS: Lazy<Vec<&'static (dyn CoordinateSystem + Send + Sync)>> =
    Lazy::new(|| vec![XY.deref(), XYZ.deref()]);

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use anyhow::{anyhow, bail, ensure, Context, Result};
    use insta::assert_ron_snapshot;

    #[test]
    fn t() -> anyhow::Result<()> {
        let coordinate_systems = COORDINATE_SYSTEMS.deref();
        let coordinate_system_names = coordinate_systems
            .iter()
            .map(|&cs| cs.name())
            .collect::<Vec<_>>();
        assert_ron_snapshot!(coordinate_system_names, @r###"
        [
          "XY",
          "XYZ",
        ]
        "###);
        Ok(())
    }
}
