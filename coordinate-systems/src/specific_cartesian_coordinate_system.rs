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

/// The definition of a specific Cartesian coordinate system.

#[derive(PartialEq, PartialOrd, Serialize)]
pub struct SpecificCartesianCoordinateSystem<const D: usize> {
    pub scs: SpecificCoordinateSystem<D>,
}

impl<const D: usize> SpecificCartesianCoordinateSystem<D> {
    pub fn debug_struct_fields<'a, 'b: 'a, 'd>(
        &self,
        debug_struct: &'d mut DebugStruct<'a, 'b>,
    ) -> &'d mut DebugStruct<'a, 'b> {
        self.scs.debug_struct_fields(debug_struct)
    }
}

impl<const D: usize> Display for SpecificCartesianCoordinateSystem<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("SpecificCartesianCoordinateSystem<{D}>");
        let mut debug_struct = f.debug_struct(&name);
        self.debug_struct_fields(&mut debug_struct).finish()
    }
}

impl<const D: usize> Debug for SpecificCartesianCoordinateSystem<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<const D: usize> CoordinateSystem for SpecificCartesianCoordinateSystem<D> {
    fn name(&self) -> Cow<'_, str> {
        self.scs.name()
    }

    fn cartesian(&self) -> bool {
        true
    }

    fn cnt_dimensions(&self) -> usize {
        D
    }

    fn dimension_info(&self, ix: usize) -> Option<&DimensionInfo> {
        self.scs.dimension_info(ix)
    }
}

impl<const D: usize> CoordinateSystemD<D> for SpecificCartesianCoordinateSystem<D> {}

impl<const D: usize> CartesianCoordinateSystem for SpecificCartesianCoordinateSystem<D> {}

impl<const D: usize> CartesianCoordinateSystemD<D> for SpecificCartesianCoordinateSystem<D> {}
