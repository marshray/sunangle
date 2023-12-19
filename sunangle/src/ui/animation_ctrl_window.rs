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
use std::ops::DerefMut;
//? use std::fmt::Display;
//? use std::ops::RangeInclusive;
use std::sync::{Arc, RwLock};

//? use anyhow::{anyhow, bail, ensure, Context, Result};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::tai::DateTimeTai;
use crate::ui::showable::ShowableEguiWindow;
use crate::view_state::{AnimationState, AnimationStateEn};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AnimationCtrlWindow {
    arcrwl_animation_state: Arc<RwLock<AnimationState>>,
}

impl AnimationCtrlWindow {
    const NAME_STR: &'static str = "Animation";

    pub fn new(arcrwl_animation_state: Arc<RwLock<AnimationState>>) -> AnimationCtrlWindow {
        AnimationCtrlWindow {
            arcrwl_animation_state,
        }
    }
}

impl ShowableEguiWindow for AnimationCtrlWindow {
    fn name(&self) -> Cow<'_, str> {
        Self::NAME_STR.into()
    }

    fn add_contents(&mut self, ui: &mut egui::Ui) {
        //let mut rate_s = "1.0 s per s".to_string();
        //let mut animation_rate = 1.0_f32;

        let (prev_en, prev_log10_animation_speed) = {
            let animation_state_guard = self.arcrwl_animation_state.read().unwrap();
            (
                animation_state_guard.en,
                animation_state_guard.log10_animation_speed,
            )
        };

        ui.horizontal(|ui| {
            if ui.button("realtime").clicked() {
                {
                    let mut animation_state_guard = self.arcrwl_animation_state.write().unwrap();
                    animation_state_guard.log10_animation_speed = 0.0;
                }
            }
        });

        #[rustfmt::skip]
        ui.horizontal(|ui| {
            ui.label("10^");

            const UPDATE_INCREMENT: f64 = 0.01;

            let mut log10_animation_speed: f64 = prev_log10_animation_speed;
            ui.add(egui::DragValue::new(&mut log10_animation_speed).speed(UPDATE_INCREMENT));

            let significant_change = UPDATE_INCREMENT/2.0 < (prev_log10_animation_speed - log10_animation_speed).abs();

            let effective_log10_animation_speed = if significant_change {
                let mut animation_state_guard = self.arcrwl_animation_state.write().unwrap();
                animation_state_guard.log10_animation_speed = log10_animation_speed;
                log10_animation_speed
            } else {
                prev_log10_animation_speed
            };

            let rate = (10.0_f64).powf(effective_log10_animation_speed);
            ui.label(format!("shows {rate} s per s"));
        });

        #[rustfmt::skip]
        ui.horizontal(|ui| {
            let is_rw = matches!(prev_en, AnimationStateEn::Rewind);
            let is_pa = matches!(prev_en, AnimationStateEn::Paused);
            let is_pl = matches!(prev_en, AnimationStateEn::Play);
            let is_ff = matches!(prev_en, AnimationStateEn::FF);

            let rw_cl = ui.add(egui::Button::new("rw")   .selected(is_rw)).clicked();
            let pa_cl = ui.add(egui::Button::new("pause").selected(is_pa)).clicked();
            let pl_cl = ui.add(egui::Button::new("play") .selected(is_pl)).clicked();
            let ff_cl = ui.add(egui::Button::new("ff")   .selected(is_ff)).clicked();

            let mut opt_en: Option<AnimationStateEn> = match (rw_cl, pa_cl, pl_cl, ff_cl) {
                (    _,  true,     _,     _) => Some(AnimationStateEn::Paused),
                (false, false,  true, false) => Some(AnimationStateEn::Play),
                ( true, false, false, false) => Some(AnimationStateEn::Rewind),
                (false, false, false,  true) => Some(AnimationStateEn::FF),
                _                            => None,
            };

            if let Some(en) = opt_en {
                info!("Animation: {en:?}");
                let mut animation_state_guard = self.arcrwl_animation_state.write().unwrap();
                animation_state_guard.en = en;
            }
        });
    }
}
