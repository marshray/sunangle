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

use anyhow::{anyhow, bail, ensure, Context, Result};
use log::{debug, error, info, trace, warn};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, SecondsFormat, TimeZone, Utc};
//use egui_extras::DatePickerButton;

use crate::gl_draw::paint_root_viewport_gl;
use crate::tai::DateTimeTai;
use crate::view_state::ViewState;
use crate::world_state::WorldState;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SunangleApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    //next_frame_number: u64,
    utc_text_edit: String,
    utc_to_tai_clicked: bool,
    tai: DateTimeTai,
}

impl Default for SunangleApp {
    fn default() -> Self {
        Self {
            value: 2.7,
            //next_frame_number: 0,
            utc_text_edit: String::new(),
            utc_to_tai_clicked: false,
            tai: WorldState::default_tai(),
        }
    }
}

impl SunangleApp {
    pub fn tai(&self) -> DateTimeTai {
        self.tai
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state
        let mut self_: Self = cc
            .storage
            .map(|storage: &dyn eframe::Storage| {
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
            })
            .unwrap_or_default();

        //self_.next_frame_number = 0;
        self_.utc_to_tai_clicked = false;
        self_.utc_text_edit = self_.tai.to_string();

        self_
    }
}

impl eframe::App for SunangleApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, eframe_frame: &mut eframe::Frame) {
        // The "next" frame begins.
        //let frame_number = self.next_frame_number;
        //self.next_frame_number += 1;

        if let Err(e) = self.update_impl(ctx, eframe_frame) {
            error!("eframe::App::update error {e}");
        }
    }

    /// Called occasionally and before shutdown. Persist the state here.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl SunangleApp {
    fn update_impl(&mut self, ctx: &egui::Context, eframe_frame: &mut eframe::Frame) -> Result<()> {
        // Update the time
        if self.utc_to_tai_clicked {
            self.utc_to_tai_clicked = false;
            //debug!("utc_to_tai_clicked");

            // Attempt to parse utc_text_edit into tai
            self.tai = self.utc_text_edit.parse()?;

            // That worked, so write tai into utc_text_edit
            info!("Assigned to TAI from 'UTC -> TAI' button");
            info!("{} -> {}", self.utc_text_edit, &self.tai);
            self.utc_text_edit = self.tai.to_utc().to_string();
            info!("{} -> {}", &self.tai, self.utc_text_edit);
        }

        //let world_state = WorldState::world_at_tai(self.tai);
        //let view_state = ViewState::new();

        // Paint the gl stuff behind the UI, log any errors.
        if let Err(e) = paint_root_viewport_gl(
            self,
            ctx,
            eframe_frame, //world_state, &view_state
        ) {
            error!("gl_draw::paint_root_viewport_gl(): error {e}");
        }

        // Now do all the egui UI stuff.

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            //eframe_frame.close();
                            error!("I don't know how to quit (eframe::Frame removed the close method)");
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            //ui.separator();
            ui.label(format!("Frame: {}", ctx.frame_nr()));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sunangle");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/marshray/sunangle/",
                "Source code."
            ));

            // let mut datepicker_value = default_date();
            // ui.add(DatePickerButton::new(&mut datepicker_value));
            //ui.add(DatePickerButton::new(&mut self.datepicker_value));
            ui.text_edit_singleline(&mut self.utc_text_edit);

            if ui.button("UTC -> TAI").clicked() {
                self.utc_to_tai_clicked = true;
            }

            ui.code(&self.tai.to_string());

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
                egui::warn_if_debug_build(ui);
            });

            //ui.separator();
            //ui.label(format!("UTC: {utc}"));
        });

        Ok(())
    }
}
