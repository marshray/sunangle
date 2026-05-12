// Copyright 2023,2026 Marsh J. Ray
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

use anyhow::{Context, Error, Result, anyhow, bail, ensure};
use log::{debug, error, info, log_enabled, trace, warn};
//? use serde::{Deserialize, Serialize};

use sunangle::SunangleApp;
use sunangle::log_error_chain;

//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(any(target_arch = "wasm32")))]
fn main() -> std::process::ExitCode {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    log::info!("logging initialized.");

    let ec = if let Err(e) = main2() {
        log_error_chain("main():", &e);
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    };

    debug!("exit code: {ec:?}");

    ec
}

#[cfg(not(any(target_arch = "wasm32")))]
fn main2() -> Result<()> {
    async_global_executor::block_on(main3()).map_err(|e| anyhow!("{e}"))?;

    debug!("Uneventful main3.");

    Ok(())
}

#[cfg(not(any(target_arch = "wasm32")))]
async fn main3() -> eframe::Result<()> {
    use eframe::CreationContext;
    use egui::ViewportBuilder;

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

    let app_creator = SunangleApp::native_app_creator_creator();

    eframe::run_native("sunangle", native_options, app_creator)
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("sunangle_canvas_id")
            .expect("Failed to find 'sunangle_canvas_id'")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("'sunangle_canvas_id' was not a HtmlCanvasElement");

        let mut web_runner = eframe::WebRunner::new();

        let start_result = web_runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(SunangleApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match &start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p>SunangleApp crashed. The web browser's developer console [F12] may have details.</p>",
                    );
                }
            }
        }

        if let Err(e) = start_result {
            log::error!("Failed to start eframe: {e:?}");
            panic!("Failed to start eframe: {e:?}");
        }

        let mut sunangle_app = web_runner.app_mut::<SunangleApp>().unwrap();

        sunangle_app.load_assets_async().await
    });
}
