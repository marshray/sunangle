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
#![allow(clippy::redundant_closure)] //? TODO for development
#![allow(clippy::too_many_arguments)]

use std::borrow::Cow;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, bail, ensure, Context, Result};
use egui::{epaint, Align, Frame, Hyperlink, Layout, Ui};
use hecs::World;
use log::{debug, error, info, trace, warn};
use serde::{self, Deserialize, Serialize};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, SecondsFormat, TimeZone, Utc};

use crate::draw_frame_info::DrawFrameInfo;
use crate::tai::DateTimeTai;
use crate::ui;
use crate::ui::showable::ShowableEguiWindow;
use crate::view_state::{AnimationState, ViewState};
use crate::world_state::{TimeState, WorldState};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct SunangleApp {
    // Ecs
    #[serde(skip)]
    world: World,

    // Ephemeral Frame stuff.
    //
    #[serde(skip)]
    draw_frame_info: DrawFrameInfo,

    // UI stuff
    //
    ui_settings_checkbox: bool,
    current_time_checkbx: bool,
    animation_checkbx: bool,
    ecs_explore_checkbx: bool,

    #[serde(skip)]
    opt_current_time_ctrl_window: Option<ui::CurrentTimeCtrlWindow>,

    #[serde(skip)]
    opt_animation_ctrl_window: Option<ui::AnimationCtrlWindow>,

    #[serde(skip)]
    opt_ecs_explore_window: Option<ui::EcsExploreWindow>,

    arcrwl_animation_state: Arc<RwLock<AnimationState>>,
    arcrwl_world_state: Arc<RwLock<WorldState>>,
    //next_frame_number: u64,
    //#[serde(skip)]

    //? TODO: world_state::TimeState
    //tai: DateTimeTai,
}

impl Default for SunangleApp {
    fn default() -> Self {
        Self {
            world: World::new(),
            draw_frame_info: DrawFrameInfo::new(),
            ui_settings_checkbox: false,
            current_time_checkbx: true,
            animation_checkbx: true,
            ecs_explore_checkbx: true,
            opt_current_time_ctrl_window: None,
            opt_animation_ctrl_window: None,
            opt_ecs_explore_window: None,
            arcrwl_animation_state: Arc::new(RwLock::new(AnimationState::default())),
            arcrwl_world_state: Arc::new(RwLock::new(WorldState::default())),
            //next_frame_number: 0,

            //tai: TimeState::default_tai(),
        }
    }
}

impl SunangleApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state
        let mut self_ = Self::load_from_storage(cc).unwrap_or_default();

        // Initialize the world.
        coordinate_systems::ecs_add_stuff(&mut self_.world);

        self_
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
}

impl eframe::App for SunangleApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, eframe_frame: &mut eframe::Frame) {
        if let Err(e) = self.draw_frame_info.start_ui_update(ctx.frame_nr()) {
            error!("eframe::App::update error start_ui_update {e}");
        }

        if let Err(e) = self.update_impl(ctx, eframe_frame) {
            error!("eframe::App::update error {e}");
        }

        self.consider_requesting_new_frame(ctx);

        if let Err(e) = self.draw_frame_info.finish_ui_update() {
            error!("eframe::App::update error finish_ui_update {e}");
        }
    }

    /// Called occasionally, and before shutdown, to persist state.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        debug!("Saving SunagleApp:\n{}", self.to_string(true));

        #[cfg(not(target_arch = "wasm32"))]
        info!("Saving...");

        eframe::set_value(storage, eframe::APP_KEY, self);

        #[cfg(not(target_arch = "wasm32"))]
        info!("saved.");
    }
}

impl SunangleApp {
    fn update_impl(&mut self, ctx: &egui::Context, eframe_frame: &mut eframe::Frame) -> Result<()> {
        self.top_panel(ctx);
        self.central_panel(ctx);

        if self.ui_settings_checkbox {
            egui::Window::new("UI Settings")
            //.open()
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });}

        if self.current_time_checkbx {
            self.opt_current_time_ctrl_window
                .get_or_insert_with(|| {
                    ui::CurrentTimeCtrlWindow::new(self.arcrwl_world_state.clone())
                })
                .show(ctx);
        }

        if self.animation_checkbx {
            self.opt_animation_ctrl_window
                .get_or_insert_with(|| {
                    ui::AnimationCtrlWindow::new(self.arcrwl_animation_state.clone())
                })
                .show(ctx);
        }

        if self.ecs_explore_checkbx {
            self.opt_ecs_explore_window
                .get_or_insert_with(|| ui::EcsExploreWindow::new())
                .show(ctx);
        }
        Ok(())
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        let tp = egui::TopBottomPanel::top("top_panel");
        tp.show(ctx, |ui| self.top_panel_add_contents(ui));
    }

    fn top_panel_add_contents(&mut self, ui: &mut Ui) {
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
                //ui.add_space(16.0);
            }
        });
        // */

        ui.horizontal(|ui| {
            ui.heading("Sunangle");
            ui.add_space(16.0);
            ui.label("Controls:");
            ui.checkbox(&mut self.ui_settings_checkbox, "UI Settings");
            ui.checkbox(&mut self.current_time_checkbx, "Time");
            ui.checkbox(&mut self.animation_checkbx, "Animation");
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        let central_panel_frame_settings = Frame::none().fill(egui::Color32::BLACK);

        let cp = egui::CentralPanel::default().frame(central_panel_frame_settings);

        cp.show(ctx, |ui| self.central_panel_add_contents(ui));
    }

    fn central_panel_add_contents(&mut self, ui: &mut Ui) {
        let tai = {
            let world_state_guard = self.arcrwl_world_state.read().unwrap();
            world_state_guard.time.tai
        };
        ui.code(tai.to_string());

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
        let arcrwl_animation_state = self.arcrwl_animation_state.clone();
        let arcrwl_world_state = self.arcrwl_world_state.clone();

        let egui_glow_callbackfn = egui_glow::CallbackFn::new(
            move |paint_callback_info: epaint::PaintCallbackInfo,
                  egui_glow_painter: &egui_glow::Painter| {
                let glow_context = egui_glow_painter.gl();
                let arcrwl_animation_state = arcrwl_animation_state.clone();
                let arcrwl_world_state = arcrwl_world_state.clone();

                //self.draw_frame_info.start_paint();

                crate::threed::threedapp::with_three_d_app(glow_context, move |threedapp| {
                    let arcrwl_animation_state = arcrwl_animation_state.clone();
                    let arcrwl_world_state = arcrwl_world_state.clone();

                    threedapp.paint_callback(
                        &paint_callback_info,
                        egui_glow_painter,
                        arcrwl_animation_state,
                        arcrwl_world_state,
                    )
                });

                //self.draw_frame_info.finish_paint();
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

    fn consider_requesting_new_frame(&mut self, ctx: &egui::Context) {
        let is_animating = {
            let ani_state = self.arcrwl_animation_state.read().unwrap();
            ani_state.is_animating()
        };

        if is_animating {
            const MAX_FRAMERATE: f64 = 60.0;
            const MIN_FRAMEDURATION: f64 = 1.0 / MAX_FRAMERATE;
            //? TODO: we should take into account how long the current frame took to draw and subtract that
            // from MAX_FRAMERATE.
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(MIN_FRAMEDURATION));
        }
    }
}
