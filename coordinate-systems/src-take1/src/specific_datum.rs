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

/// The definition of a specific coordinate system.
#[derive(PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SpecificDatum {
    pub name: &'static str,
    pub urls: Vec<(&'static str, &'static str)>,
    pub opt_frame_reference_epoch: Option<CowStaticStr>,
}

impl SpecificDatum {
    pub fn debug_struct_fields<'a, 'b: 'a, 'd>(
        &self,
        debug_struct: &'d mut DebugStruct<'a, 'b>,
    ) -> &'d mut DebugStruct<'a, 'b> {
        debug_struct
            .field("name", &self.name)
            .field("urls", &self.urls)
            .field("opt_frame_reference_epoch", &self.opt_frame_reference_epoch)
    }
}

impl Display for SpecificDatum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("SpecificDatum");
        self.debug_struct_fields(&mut debug_struct).finish()
    }
}

impl Debug for SpecificDatum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }

}

impl Name for SpecificDatum {
    fn name(&self) -> CowStaticStr {
        self.name.into()
    }
}

impl Urls for SpecificDatum {
    fn urls(&self) -> &dyn Deref<Target = [(CowStaticStr, CowStaticStr)]> {
        &self.urls
    }
}

impl Datum for SpecificDatum {
}

impl DynamicDatum for SpecificDatum {
    fn frame_reference_epoch(&self) -> CowStaticStr {
        &self.opt_frame_reference_epoch.unwrap_or_default()
    }
}
