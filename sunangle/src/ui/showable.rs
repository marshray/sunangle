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
#![allow(clippy::too_many_arguments)]

//? use std::any::Any;
use std::borrow::Cow;
//? use std::fmt::Display;
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use serde::{Deserialize, Serialize};

use egui::{Context, Ui};
pub trait ShowableEguiWindow {
    fn name(&self) -> Cow<'_, str>;

    fn add_contents(&mut self, ui: &mut Ui);

    fn show(&mut self, ctx: &Context) {
        //debug!("{}::Displayable::new_show", self.name());
        egui::Window::new(self.name()).show(ctx, |ui| {
            self.add_contents(ui);
        });
    }
}
