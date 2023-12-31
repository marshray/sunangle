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
//? use std::fmt::{Debug, Display};
//? use std::ops::{RangeBounds, RangeInclusive};
//? use std::sync::Arc;
//? use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use derive_more::{Deref, DerefMut, Display, From, Into};
use hecs::{Bundle, Entity, World};
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use egui::{Context, Ui};

pub trait ShowableEguiWindow {
    fn name(&self) -> Cow<'_, str>;

    fn add_contents(&mut self, ui: &mut Ui, world: &mut World);

    fn show(&mut self, ctx: &Context, world: &mut World) {
        //debug!("{}::ShowableEguiWindow::show", self.name());
        egui::Window::new(self.name()).show(ctx, |ui| {
            self.add_contents(ui, world);
        });
    }
}
