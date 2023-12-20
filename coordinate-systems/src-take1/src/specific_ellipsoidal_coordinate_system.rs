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
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::*;

/// The definition of a specific Ellipsoid3.
/// 
#[derive(PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SpecificEllipsoid3 {
    pub name: CowStaticStr,
    pub urls: Vec<(CowStaticStr, CowStaticStr)>,
    pub structure: CSStructure,
    pub dim_infos: Vec<DimensionInfo>,
}

impl SpecificEllipsoid3 {
    pub fn debug_struct_fields<'a, 'b: 'a, 'd>(
        &self,
        debug_struct: &'d mut DebugStruct<'a, 'b>,
    ) -> &'d mut DebugStruct<'a, 'b> {
        debug_struct
            .field("name", &self.name)
            .field("urls", &self.urls)
            .field("structure", &self.CSStructure)
            .field("dim_infos", &self.dim_infos)
    }
}

impl Display for SpecificEllipsoid3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("SpecificEllipsoid<{D}>");
        let mut debug_struct = f.debug_struct(&name);
        self.debug_struct_fields(&mut debug_struct).finish()
    }
}

impl Debug for SpecificEllipsoid3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Name for SpecificEllipsoid3 {
    fn name(&self) -> &CowStaticStr {
        self.name.into()
    }
}

impl Urls for SpecificEllipsoid3 {
    fn urls(&self) -> &dyn Deref<Target = [(CowStaticStr, CowStaticStr)]> {
        &self.urls
    }
}

impl CoordinateSystem for SpecificEllipsoid3 {
    fn cs_structure(&self) -> CSStructure {
        self.structure
    }

    fn cnt_dimensions(&self) -> usize {
        D
    }

    fn dimension_info(&self, ix: usize) -> Option<&DimensionInfo> {
        assert_eq!(self.dim_infos.len(), D);
        if ix < D {
            Some(&self.dim_infos[ix])
        } else {
            None
        }
    }
}

/// See [IOGP 373-7-2 Geomatics Guidance number 7, part 2](https://epsg.org/guidance-notes.html)
/// for details on Ellipsoid3 calculations.
#[derive(PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SpecificEllipsoid3alCoordinateSystem {
    pub scs: SpecificEllipsoid<3>,
}

impl SpecificEllipsoid3alCoordinateSystem {
    pub const CNT_DIMENSIONS: usize = 3;

    pub fn debug_struct_fields<'a, 'b: 'a, 'd>(
        &self,
        debug_struct: &'d mut DebugStruct<'a, 'b>,
    ) -> &'d mut DebugStruct<'a, 'b> {
        self.scs.debug_struct_fields(debug_struct)
    }
}

impl Display for SpecificEllipsoid3alCoordinateSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("SpecificEllipsoid3CoordinateSystem");
        self.debug_struct_fields(&mut debug_struct).finish()
    }
}

impl Debug for SpecificEllipsoid3alCoordinateSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Name for SpecificEllipsoid3alCoordinateSystem {
    fn name(&self) -> CowStaticStr {
        self.name.into()
    }
}

impl Urls for SpecificEllipsoid3alCoordinateSystem {
    fn urls(&self) -> &dyn Deref<Target = [(CowStaticStr, CowStaticStr)]> {
        &self.urls
    }
}

impl CoordinateSystem for SpecificEllipsoid3alCoordinateSystem {
    fn cs_structure(&self) -> CSStructure {
        debug_assert_eq!(self.scs.cs_structure(), CSStructure::Cartesian);
        CSStructure::Ellipsoid
    }

    fn cnt_dimensions(&self) -> usize {
        Self::CNT_DIMENSIONS
    }

    fn dimension_info(&self, ix: usize) -> Option<&DimensionInfo> {
        self.scs.dimension_info(ix)
    }
}

impl CoordinateSystemD<3> for SpecificEllipsoid3alCoordinateSystem {}

impl EllipsoidalCoordinateSystem for SpecificEllipsoid3alCoordinateSystem {}

impl Ellipsoid3CoordinateSystem for SpecificEllipsoid3alCoordinateSystem {}

