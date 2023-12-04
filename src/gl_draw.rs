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

//? use use std::fmt::Display;
use std::any::Any;
use std::cell::RefCell;
use std::sync::Arc;
use std::rc::Rc;

use anyhow::{anyhow, bail, ensure, Result};
use log::{debug, error, info, trace, warn};
//? use serde::{Deserialize, Serialize};
use eframe::egui_glow;
use eframe::glow;
use egui::epaint;

use crate::eframe_app::SunangleApp;
use crate::tai::DateTimeTai;
use crate::view_state::ViewState;
use crate::world_state::WorldState;

pub fn paint_root_viewport_gl(
    app: &mut SunangleApp,
    ctx: &egui::Context,
    eframe_frame: &mut eframe::Frame,
    //world_state: WorldState,
    //view_state: &ViewState,
) -> Result<()> {
    //y We're not doing it this way, because we want eframe to drive the loop
    /* // Obtain an Arc on the `glow::Context`
    let arc_glow_context = eframe_frame
        .gl()
        .ok_or_else(|| anyhow!("Couldn't get glow gl context"))?.clone();
    //let painter = egui::Painter::new(ctx: Context, layer_id: LayerId, clip_rect: Rect);

    // Construct a `three_d::GUI` from the `glow::Context`
    let gui = three_d::GUI::new(&arc_glow_context);
    // */

    // Set up a weird callback system for painting the background layer.

    //let backgroud_layer_id = egui::LayerId::background();
    //let layer_painter = ctx.layer_painter(backgroud_layer_id);
    let layer_painter = ctx.debug_painter(); //?

    //let rect = egui::Rect::EVERYTHING;
    let clip_rect = layer_painter.clip_rect();

    //? let mut world_state = world_state;

    let tai = app.tai();

    let egui_glow_callbackfn = egui_glow::CallbackFn::new(
        move |paint_callback_info: epaint::PaintCallbackInfo,
              egui_glow_painter: &egui_glow::Painter| {
            let glow_context = egui_glow_painter.gl();

            with_three_d_app(glow_context, |threedapp| {
                let input_translator = InputTranslator::new(
                    &paint_callback_info,
                    egui_glow_painter,
                    &threedapp.core_context,
                );

                threedapp.paint_callback(input_translator, tai);
            });
        },
    );

    let background_layer_paint_callback = egui::PaintCallback {
        rect: clip_rect,
        callback: Arc::new(egui_glow_callbackfn),
    };

    let shape = egui::Shape::Callback(background_layer_paint_callback);
    layer_painter.add(shape);

    Ok(())
}

fn with_three_d_app<R>(
    arc_glow_context: &Arc<glow::Context>,
    f: impl Fn(&mut ThreeDApp) -> R,
) -> R {
    thread_local! {
        static REFCELL_OPT_THREEDAPP: RefCell<Option<ThreeDApp>> = RefCell::default();
    }

    REFCELL_OPT_THREEDAPP.with(|refcell_opt_threedapp| {
        let mut opt_threedapp = refcell_opt_threedapp.borrow_mut();

        let threedapp =
            opt_threedapp.get_or_insert_with(|| ThreeDApp::new(arc_glow_context.clone()));

        f(threedapp)
    })
}

use three_d::material::ColorMaterial;
use three_d::renderer::{Camera, Gm, Mesh};
use three_d::{
    degrees, radians, vec3, ClearState, CpuMesh, Deg, Mat4, Positions, RenderTarget, ScissorBox, Srgba,
    Viewport,
};

pub struct ThreeDApp {
    core_context: three_d::core::Context,
    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
    triangle_rotate: Deg<f32>,
}

impl ThreeDApp {
    
    pub fn new(arc_glow_context: Arc<glow::Context>) -> Self {
        debug!("ThreeDApp::new(...)");

        let core_context = three_d::core::Context::from_gl_context(arc_glow_context).unwrap();

        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1), //x ????
            vec3(0.0, 0.0, 2.0),          //x ????
            vec3(0.0, 0.0, 0.0),          //x ????
            vec3(0.0, 1.0, 0.0),          //x ????
            degrees(45.0),                //x ????
            0.1,
            10.0,
        );

        // Create a CPU-side mesh consisting of a single colored triangle //x ????
        let positions = vec![
            vec3(0.5, -0.5, 0.0),  // bottom right //x ????
            vec3(-0.5, -0.5, 0.0), // bottom left //x ????
            vec3(0.0, 0.5, 0.0),   // top //x ????
        ];

        let colors = vec![
            Srgba::new(255, 0, 0, 255), // bottom right //x ????
            Srgba::new(0, 255, 0, 255), // bottom left //x ????
            Srgba::new(0, 0, 255, 255), // top //x ????
        ];

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions), //x ????
            colors: Some(colors),                 //x ????
            ..Default::default()                  //x ????
        };

        // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
        let model = Gm::new(
            Mesh::new(&core_context, &cpu_mesh),
            ColorMaterial::default(),
        );

        Self {
            core_context,
            camera,
            model,
            triangle_rotate: degrees(123.0),
        }
    }

    pub fn paint_callback(
        &mut self,
        input_translator: InputTranslator<'_>,
        tai: DateTimeTai,
    ) -> Option<glow::Framebuffer> {
        let tri_rot_y = self.triangle_rotate;
        debug!("threedapp.paint_cb: triangle_rotate: {tri_rot_y:?}");

        self.triangle_rotate += degrees(1.0);

        // Figure the world state from tai.
        //let world_state = WorldState::world_at_tai(tai);
        //debug!("t: {tai},   triangle_rotate: {:?} deg", Deg::<f32>::from(triangle_rotate));

        self.camera.set_viewport(input_translator.viewport);

        // Set the current transformation of the triangle //x ????
        //let triangle_rotate = radians(0.0_f32
        //    + tai.to_utc().timestamp_subsec_millis() as f32/1000.0 //?
        //);

        self.model
            .set_transformation(Mat4::from_angle_y(tri_rot_y));

        // Get the screen render target to be able to render something on the screen //x ????
        input_translator
            .render_target
            // Clear the color and depth of the screen render target //x ????
            .clear_partially(
                input_translator.scissor_box,
                ClearState::depth(1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction //x ????
            .render_partially(
                input_translator.scissor_box,
                &self.camera,
                [&self.model],
                &[],
            );

            // Take back the screen fbo, we will continue to use it.
        input_translator.render_target.into_framebuffer()
    }
}

// Translate from the egui input system to the three_d input system.
pub struct InputTranslator<'a> {
    render_target: RenderTarget<'a>,
    viewport: Viewport,
    scissor_box: ScissorBox,
}

impl InputTranslator<'_> {
    pub fn new(
        paint_info: &epaint::PaintCallbackInfo,
        egui_glow_painter: &egui_glow::Painter,
        context: &three_d::core::Context,
    ) -> Self {
        //? TODO why?
        // Disable sRGB textures for three-d
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            use glow::HasContext as _;
            context.disable(glow::FRAMEBUFFER_SRGB);
        }

        Self {
            render_target: Self::rendertarget_from_paint_info(paint_info, egui_glow_painter, context),
            viewport: Self::viewport_from_paint_info(paint_info),
            scissor_box: Self::scissor_box_from_paint_info(paint_info),
        }
    }

    fn viewport_from_paint_info(paint_info: &epaint::PaintCallbackInfo) -> three_d::Viewport {
        let vp_px = paint_info.viewport_in_pixels();

        Viewport {
            x: vp_px.left_px,
            y: vp_px.from_bottom_px,
            width: vp_px.width_px as _,
            height: vp_px.height_px as _,
        }
    }

    fn scissor_box_from_paint_info(paint_info: &epaint::PaintCallbackInfo) -> three_d::ScissorBox {
        let clip_rect = paint_info.clip_rect_in_pixels();

        ScissorBox {
            x: clip_rect.left_px,
            y: clip_rect.from_bottom_px,
            width: clip_rect.width_px as _,
            height: clip_rect.height_px as _,
        }
    }

    fn rendertarget_from_paint_info<'a>(
        paint_info: &epaint::PaintCallbackInfo,
        egui_glow_painter: &egui_glow::Painter,
        context: &three_d::core::Context,
    ) -> three_d::RenderTarget<'a> {
        let w: u32 = paint_info.viewport.width().round() as _;
        let h: u32 = paint_info.viewport.height().round() as _;
        if let Some(fbo) = egui_glow_painter.intermediate_fbo() {
            RenderTarget::from_framebuffer(context, w, h, fbo)
        } else {
            RenderTarget::screen(context, w, h)
        }
    }
}
