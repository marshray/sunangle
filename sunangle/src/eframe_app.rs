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

use std::borrow::Cow;
use std::sync::Arc;

use anyhow::{anyhow, bail, ensure, Context, Result};
use egui::{epaint, Align, Frame, Hyperlink, Layout, Ui};
use log::{debug, error, info, trace, warn};
use serde::{self, Deserialize, Serialize};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, SecondsFormat, TimeZone, Utc};

use crate::tai::DateTimeTai;
use crate::ui::showable::ShowableEguiWindow;
use crate::ui::time_ctrl_window::TimeCtrlWindow;
use crate::view_state::ViewState;
use crate::world_state::WorldState;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct SunangleApp {
    current_time_checkbx: bool,
    #[serde(skip)]
    opt_current_time_window: Option<TimeCtrlWindow>,

    //next_frame_number: u64,
    #[serde(skip)]
    utc_text_edit: String,

    tai: DateTimeTai,
}

impl Default for SunangleApp {
    fn default() -> Self {
        Self {
            current_time_checkbx: true,
            opt_current_time_window: None,

            //next_frame_number: 0,
            utc_text_edit: String::new(),

            tai: WorldState::default_tai(),
        }
    }
}

impl SunangleApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state
        Self::load_from_storage(cc).unwrap_or_default()
    }

    fn load_from_storage(cc: &eframe::CreationContext<'_>) -> Option<Self> {
        let Some(storage) = cc.storage else {
            debug!("Loading SunagleApp: No storage");
            return None;
        };

        let Some(str_self) = storage.get_string(eframe::APP_KEY) else {
            debug!("Loading SunagleApp: Storage exists, but APP_KEY is empty");
            return None;
        };

        match ron::from_str::<Self>(&str_self) {
            Err(e) => {
                warn!("Loading SunagleApp: decode err: {e}");
                None
            }
            Ok(mut self_) => {
                debug!("Loaded SunagleApp:\n{}", self_.to_string(true));
                Some(self_)
            }
        }
    }

    fn to_string(&self, pretty: bool) -> String {
        if pretty {
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
        } else {
            ron::ser::to_string(self)
        }
        .unwrap_or_default()
    }

    pub fn tai(&self) -> DateTimeTai {
        self.tai
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

    /// Called occasionally, and before shutdown, to persist state.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        debug!("Saving SunagleApp:\n{}", self.to_string(true));

        info!("Saving...");
        eframe::set_value(storage, eframe::APP_KEY, self);
        info!("saved.");
    }
}

impl SunangleApp {
    fn update_impl(&mut self, ctx: &egui::Context, eframe_frame: &mut eframe::Frame) -> Result<()> {
        // Update the time

        /* if let Some(Some(())) = self
            .opt_current_time_window
            .as_mut()
            .map(TimeCtrlWindow::take_utc_to_tai_click)
        {
            debug!("utc_to_tai_clicked");
        } */

        if let Some(tai) = self
            .opt_current_time_window
            .as_mut()
            .and_then(|ctw| ctw.take_updated_tai(self.tai))
        {
            debug!("updated tai");
            self.tai = tai;
        }

        //let world_state = WorldState::world_at_tai(self.tai);
        //let view_state = ViewState::new();

        // Now do all the egui UI stuff.

        self.top_panel(ctx);
        self.central_panel(ctx);

        if self.current_time_checkbx {
            self.opt_current_time_window
                .get_or_insert_with(|| TimeCtrlWindow::new(self.tai))
                .show(ctx);
        }

        Ok(())
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        let tp = egui::TopBottomPanel::top("top_panel");

        tp.show(ctx, |ui| self.top_panel_add_contents(ui));
    }

    fn top_panel_add_contents(&mut self, ui: &mut Ui) {
        ui.heading("Sunangle");

        /*
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
        */

        ui.horizontal(|ui| {
            ui.label("Controls:");
            ui.checkbox(&mut self.current_time_checkbx, "Time");
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        let central_panel_frame_settings = Frame::none().fill(egui::Color32::BLACK);

        let cp = egui::CentralPanel::default().frame(central_panel_frame_settings);

        cp.show(ctx, |ui| self.central_panel_add_contents(ui));
    }

    fn central_panel_add_contents(&mut self, ui: &mut Ui) {
        /*
        // let mut datepicker_value = default_date();
        // ui.add(DatePickerButton::new(&mut datepicker_value));
        //ui.add(DatePickerButton::new(&mut self.datepicker_value));
        //ui.separator();
        //ui.label(format!("UTC: {utc}"));
         */

        ui.code(&self.tai.to_string());

        ui.with_layout(Layout::left_to_right(egui::Align::BOTTOM), |ui| {
            ui.horizontal(|ui| {
                ui.add(Hyperlink::from_label_and_url(
                    "Source code",
                    "https://github.com/marshray/sunangle/tree/main/src",
                ));
            });

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
        });

        self.central_panel_set_up_paint_callback(ui);
    }

    // Request a callback to paint the central panel using `threedapp`.
    fn central_panel_set_up_paint_callback(&mut self, ui: &mut Ui) {
        let egui_glow_callbackfn = egui_glow::CallbackFn::new(
            |paint_callback_info: epaint::PaintCallbackInfo,
             egui_glow_painter: &egui_glow::Painter| {
                let glow_context = egui_glow_painter.gl();

                crate::threed::threedapp::with_three_d_app(glow_context, move |threedapp| {
                    threedapp.paint_callback(&paint_callback_info, egui_glow_painter)
                });
            },
        );

        //let rect = egui::Rect::EVERYTHING;
        let painter = ui.painter();
        let clip_rect = painter.clip_rect();

        let paint_callback = egui::PaintCallback {
            rect: clip_rect,
            callback: Arc::new(egui_glow_callbackfn),
        };

        let shape = egui::Shape::Callback(paint_callback);
        painter.add(shape);
    }
}
