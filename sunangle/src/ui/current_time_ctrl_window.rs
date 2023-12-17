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
//? use std::fmt::Display;
//? use std::ops::RangeInclusive;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, bail, ensure, Context, Result};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::tai::DateTimeTai;
use crate::ui::showable::ShowableEguiWindow;
use crate::world_state::WorldState;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CurrentTimeCtrlWindow {
    arcrwl_world_state: Arc<RwLock<WorldState>>,
    utc_text_edit: String,
    utc_to_tai_click: Option<()>,
}

impl CurrentTimeCtrlWindow {
    const NAME_STR: &str = "Time";

    pub fn new(arcrwl_world_state: Arc<RwLock<WorldState>>) -> CurrentTimeCtrlWindow {
        let tai = {
            let world_state_guard = arcrwl_world_state.read().unwrap();
            world_state_guard.time.tai
        };

        CurrentTimeCtrlWindow {
            arcrwl_world_state,
            utc_text_edit: tai.to_string(),
            utc_to_tai_click: None,
        }
    }

    //pub fn set_tai(&mut self, tai: DateTimeTai) {
    //    self.utc_text_edit = tai.to_string();
    //}

    /* pub fn take_updated_tai(&mut self, current_tai: DateTimeTai) -> Option<DateTimeTai> {
        self.utc_to_tai_click.take()?;

        //current_tai
        //    .checked_add_days(chrono::Days::new(1))
        //    .and_then(|tai| if tai != current_tai { Some(tai) } else { None })

        // Attempt to parse utc_text_edit into tai
        let new_tai: DateTimeTai = self.utc_text_edit.parse().map_err(|e| error!("{e}")).ok()?;

        // That worked, so write `new_tai` into utc_text_edit
        //info!("Assigned to TAI from 'UTC -> TAI' button");
        //info!("{} -> {new_tai}", self.utc_text_edit);
        self.utc_text_edit = new_tai.to_string();
        //info!("{new_tai} -> {}", self.utc_text_edit);

        // Write `new_tai` into world state
        {
            let mut mutref_world_state = self.arcrwl_world_state.write().unwrap();
            mutref_world_state.time.tai = new_tai;
        };

        Some(new_tai)
    } */

    fn on_clicked_set(&mut self) {
        if let Err(e) = self.on_clicked_set2() {
            error!("CurrentTimeCtrlWindow::on_clicked_set: {e}");
        }
    }

    fn on_clicked_set2(&mut self) -> Result<()> {
        // Attempt to parse utc_text_edit into tai
        let new_tai: DateTimeTai = self.utc_text_edit.parse()?;

        // That worked, so write `new_tai` into utc_text_edit

        self.assign_new_tai(new_tai)
    }

    fn on_clicked_now(&mut self) {
        if let Err(e) = self.on_clicked_now2() {
            error!("CurrentTimeCtrlWindow::on_clicked_now: {e}");
        }
    }

    fn on_clicked_now2(&mut self) -> Result<()> {
        let new_tai = DateTimeTai::now();
        self.assign_new_tai(new_tai)
    }

    fn assign_new_tai(&mut self, new_tai: DateTimeTai) -> Result<()> {
        // That worked, so write `new_tai` into utc_text_edit
        //info!("Assigned to TAI from 'UTC -> TAI' button");
        //info!("{} -> {new_tai}", self.utc_text_edit);
        self.utc_text_edit = new_tai.to_string();
        //info!("{new_tai} -> {}", self.utc_text_edit);

        // Write `new_tai` into world state
        self.arcrwl_world_state.write().unwrap().time.tai = new_tai;

        Ok(())
    }
}

impl ShowableEguiWindow for CurrentTimeCtrlWindow {
    fn name(&self) -> Cow<'_, str> {
        Self::NAME_STR.into()
    }

    fn add_contents(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.utc_text_edit);

            if ui.button("set").clicked() {
                self.on_clicked_set();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("now").clicked() {
                self.on_clicked_now();
            }
        });
    }
}
