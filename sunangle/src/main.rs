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
#![warn(clippy::all, rust_2018_idioms)]
#![allow(clippy::new_without_default)] //? TODO for development
#![allow(clippy::too_many_arguments)]

//? use use std::fmt::Display;
//? use std::ops::RangeInclusive;

//? use anyhow::{anyhow, bail, Context, Result};
//? use serde::{Deserialize, Serialize};

use sunangle::SunangleApp;

//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(any(target_arch = "wasm32")))]
fn main() -> eframe::Result<()> {
    use egui::ViewportBuilder;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    log::info!("logging initialized.");

    let window_builder_hook =
        Box::new(|mut viewport_builder: ViewportBuilder| -> ViewportBuilder {
            // Fix some weird window position issue on native Win32
            //? TODO bug this upstream?
            viewport_builder.position = Some(egui::pos2(100.0, 100.0));

            viewport_builder
        });

    let native_options = eframe::NativeOptions {
        //initial_window_size: Some([400.0, 300.0].into()),
        //min_window_size: Some([300.0, 220.0].into()),
        window_builder: Some(window_builder_hook),
        ..Default::default()
    };

    eframe::run_native(
        "sunangle",
        native_options,
        Box::new(|cc| Box::new(SunangleApp::new(cc))),
    )?;

    log::info!("Uneventful exit.");

    Ok(())
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(SunangleApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
