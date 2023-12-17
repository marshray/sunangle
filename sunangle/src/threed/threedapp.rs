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
#![allow(clippy::too_many_arguments)]

//? use use std::fmt::Display;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, bail, ensure, Result};
use log::{debug, error, info, trace, warn};
//? use serde::{Deserialize, Serialize};

use eframe::{egui_glow, glow};
use egui::epaint;

use three_d::material::ColorMaterial;
use three_d::renderer::{Camera, Gm, Mesh};
use three_d::{
    degrees, radians, vec3, ClearState, Context, CpuMaterial, CpuMesh, Deg, Geometry,
    InstancedMesh, Mat4, Object, PhysicalMaterial, Positions, RenderTarget, ScissorBox, Srgba,
    Viewport,
};
use three_d_asset::{Matrix4, PbrMaterial};

use crate::tai::DateTimeTai;
use crate::view_state::{AnimationState, ViewState};
use crate::world_state::WorldState;

pub fn with_three_d_app<R>(
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

/* trait ObjectGeometry: Object + Geometry + std::ops::Deref<Target = Mesh> {}
impl<T> ObjectGeometry for T
where
    T: Object,
    T: Geometry,
    T: std::ops::Deref<Target = Mesh>,
    T: Sized,
{ } */

pub struct ThreeDApp {
    core_context: three_d::core::Context,
    camera: Camera,
    opt_object_triangle: Option<Gm<Mesh, ColorMaterial>>,
    opt_gm_mesh_color: Option<Gm<Mesh, ColorMaterial>>,
    opt_gm_mesh_phys: Option<Gm<Mesh, PhysicalMaterial>>,
    triangle_rotate: Deg<f32>,
}

impl ThreeDApp {
    pub fn new(arc_glow_context: Arc<glow::Context>) -> Self {
        debug!("ThreeDApp::new(...)");

        // Construct a `three_d::GUI` from the `glow::Context`
        //let gui = three_d::GUI::new(&arc_glow_context);

        let core_context = three_d::core::Context::from_gl_context(arc_glow_context).unwrap();

        let viewport = Viewport::new_at_origo(1, 1);

        let camera_pos_x = 0.0;
        let camera_pos_y = 0.0;
        let camera_pos_z = -4.0;

        let camera_tgt_x = 0.0;
        let camera_tgt_y = 0.0;
        let camera_tgt_z = 0.0;

        //? TODO: is this a unit orientation vector, or relative to camera pos?
        let up_x = 0.0;
        let up_y = 1.0;
        let up_z = 0.0;

        // Near and far clipping planes. Must be nonnegative.
        let z_near = 0.1;
        //let z_far = f32::MAX / 1000.0; // practically forever
        let z_far = 100.0;

        // Field of view.
        let fov_y = degrees(40.0);

        let camera = Camera::new_perspective(
            viewport,
            vec3(camera_pos_x, camera_pos_y, camera_pos_z),
            vec3(camera_tgt_x, camera_tgt_y, camera_tgt_z),
            vec3(up_x, up_y, up_z),
            fov_y,
            z_near,
            z_far,
        );

        let opt_object_triangle = Some(Self::make_basic_triangle_model(&core_context));
        let opt_gm_mesh_color = Some(Self::make_sphere_model(&core_context));
        let opt_gm_mesh_phys = Some(Self::make_opaque_model(&core_context));

        Self {
            core_context,
            camera,
            opt_object_triangle,
            opt_gm_mesh_color,
            opt_gm_mesh_phys,
            triangle_rotate: degrees(123.0),
        }
    }

    fn make_basic_triangle_model(context: &Context) -> Gm<Mesh, ColorMaterial> {
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

        Gm::new(Mesh::new(context, &cpu_mesh), ColorMaterial::default())
    }

    fn make_sphere_model(context: &Context) -> Gm<Mesh, ColorMaterial> {
        let mut cpu_mesh = CpuMesh::sphere(6);

        // Transform from radius 1.0 to diameter 1.0.
        let _ = cpu_mesh.transform(&Matrix4::from_scale(0.5));

        // Assign colors.
        cpu_mesh.colors = Some(if let Positions::F32(ref mut ps) = cpu_mesh.positions {
            //let mut colors = vec![color; ps.len()];
            let mut colors = Vec::with_capacity(ps.len());

            for (v_ix, pos) in ps.iter().enumerate() {
                // 0.0 <= RGB <= 1.0
                let r = 0.5 + pos.y;
                let g = 0.5 + pos.y;
                let b = 0.5 + pos.y;

                if !(0.0..=1.0).contains(&r) {
                    error!("r = {r}");
                }
                if !(0.0..=1.0).contains(&g) {
                    error!("g = {g}");
                }
                if !(0.0..=1.0).contains(&b) {
                    error!("b = {b}");
                }

                /*
                //let invert_x = ((pos.x * 10.0) as u8) & 1;
                //let invert_x = ((invert_x << 1) as f32) - 1.0;
                //let invert_y = ((pos.y * 10.0) as u8) & 1;
                //let invert_y = ((invert_y << 1) as f32) - 1.0;
                //let invert_z = ((pos.z * 10.0) as u8) & 1;
                //let invert_z = ((invert_z << 1) as f32) - 1.0;

                let r = 0.0;
                let g = 0.5 + pos.z;
                let b = 0.0;
                */

                /*
                // -1.0 <= RGB <= 1.0
                let r = r * 2.0 - 1.0;
                let g = g * 2.0 - 1.0;
                let b = b * 2.0 - 1.0;
                */

                //let r = r/4.0;

                /*
                let p = 0.2;
                let t = g;
                let g = 2.0 * (t / p - (0.5 + t / p).floor());
                let g = (1.0 + g) / 2.0;
                // */

                /*
                //let r = r * invert_z;
                //let g = g * invert_x;
                //let b = b * invert_y;
                 */

                let r = ((r * 256.0) as u8).min(255);
                let g = ((g * 256.0) as u8).min(255);
                let b = ((b * 256.0) as u8).min(255);

                let c = Srgba::new_opaque(r, g, b);
                colors.push(c);
            }

            colors
        } else {
            vec![Srgba::new_opaque(0, 255, 0); cpu_mesh.positions.len()]
        });

        let geometry = Mesh::new(context, &cpu_mesh);

        let color_material = ColorMaterial::default();

        Gm::new(geometry, color_material)
    }

    fn make_opaque_model(context: &Context) -> Gm<Mesh, PhysicalMaterial> {
        /*
                let rot_z90 = Mat4::from_angle_z(Deg(90.0)); //?xxx

                // Instances initially ordered such that drawing them in order without reordering is incorrect. //?xxx
                let transparent_instances = three_d::renderer::geometry::Instances {
                    //?xxx
                    transformations: vec![
                        Mat4::from_translation(vec3(0.0, 0.0, -2.0)), //?xxx
                        Mat4::from_translation(vec3(0.0, 0.0, -1.0)), //?xxx
                        Mat4::from_translation(vec3(0.0, 0.0, 0.0)),  //?xxx
                        Mat4::from_translation(vec3(0.0, 0.0, 1.0)),  //?xxx
                        // The next two cubes always intersect, even if ordered by depth, they will show //?xxx
                        // rendering artifacts from one view-direction. //?xxx
                        Mat4::from_translation(vec3(3.0, 0.0, 0.0))
                            * rot_z90
                            * Mat4::from_angle_x(Deg(45.0)), //?xxx
                        Mat4::from_translation(vec3(3.0, 0.0, 0.5))
                            * rot_z90
                            * Mat4::from_angle_x(Deg(-45.0)), //?xxx
                    ], //?xxx
                    colors: Some(vec![
                        //?xxx
                        Srgba::new(255, 255, 255, 128), //?xxx
                        Srgba::new(255, 0, 255, 128),   //?xxx
                        Srgba::new(255, 0, 0, 128),     //?xxx
                        Srgba::new(0, 0, 255, 128),     //?xxx
                        // Next two always intersect. //?xxx
                        Srgba::new(255, 255, 255, 128), //?xxx
                        Srgba::new(0, 0, 255, 128),     //?xxx
                    ]), //?xxx
                    ..Default::default() //?xxx
                }; //?xxx

                // For opaque meshes, the draw order does not matter. //?xxx
                let opaque_instances = three_d::renderer::geometry::Instances {
                    //?xxx
                    transformations: transparent_instances.transformations, //?xxx
                    colors: Some(
                        //?xxx
                        transparent_instances //?xxx
                            .colors //?xxx
                            .as_ref() //?xxx
                            .unwrap() //?xxx
                            .iter() //?xxx
                            .map(|c| Srgba {
                                //?xxx
                                r: c.r, //?xxx
                                g: c.g, //?xxx
                                b: c.b, //?xxx
                                a: 255, //?xxx
                            }) //?xxx
                            .collect(), //?xxx
                    ), //?xxx
                    ..Default::default()                                    //?xxx
                }; //?xxx
        */
        let mut thin_cube = CpuMesh::cube(); //?xxx
        thin_cube //?xxx
            .transform(&Mat4::from_nonuniform_scale(1.0, 1.0, 0.04)) //?xxx
            .unwrap(); //?xxx

        /*
        let mut opaque_models = Gm::new(
            //?xxx
            InstancedMesh::new(context, &opaque_instances, &thin_cube), //?xxx
            PhysicalMaterial::new_opaque(
                //?xxx
                context, //?xxx
                &CpuMaterial {
                    //?xxx
                    albedo: Srgba::WHITE, //?xxx
                    ..Default::default()  //?xxx
                }, //?xxx
            ), //?xxx
        ); //?xxx

        //opaque_models.set_transformation(Mat4::from_translation(vec3(-6.0, 0.0, 0.0))); //?xxx
        */

        let mut opaque_model = Gm::new(
            //?xxx
            Mesh::new(context, &thin_cube), //?xxx
            PhysicalMaterial::new_opaque(
                //?xxx
                context, //?xxx
                &CpuMaterial {
                    //?xxx
                    albedo: Srgba::new(128, 128, 128, 255), //?xxx
                    ..Default::default()                    //?xxx
                }, //?xxx
            ), //?xxx
        ); //?xxx

        //opaque_model.set_transformation(Mat4::from_translation(vec3(0.0, -0.4, -3.0))); //?xxx

        opaque_model
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

    pub fn rendertarget_from_paint_info<'a>(
        &self,
        paint_callback_info: &epaint::PaintCallbackInfo,
        egui_glow_painter: &egui_glow::Painter,
    ) -> three_d::RenderTarget<'a> {
        let w: u32 = paint_callback_info.viewport.width().round() as _;
        let h: u32 = paint_callback_info.viewport.height().round() as _;

        if let Some(fbo) = egui_glow_painter.intermediate_fbo() {
            RenderTarget::from_framebuffer(&self.core_context, w, h, fbo)
        } else {
            RenderTarget::screen(&self.core_context, w, h)
        }
    }

    pub fn paint_callback(
        &mut self,
        paint_callback_info: &epaint::PaintCallbackInfo,
        egui_glow_painter: &egui_glow::Painter,
        arcrwl_animation_state: Arc<RwLock<AnimationState>>,
        arcrwl_world_state: Arc<RwLock<WorldState>>,
    ) // -> Option<glow::Framebuffer>
    {
        //? TODO why?
        // Disable sRGB textures for three-d
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            use glow::HasContext as _;
            self.core_context.disable(glow::FRAMEBUFFER_SRGB);
        }

        let tri_rot_y = self.triangle_rotate;
        //trace!("threedapp.paint_cb: triangle_rotate: {tri_rot_y:?}");

        self.triangle_rotate += degrees(1.0);

        // Figure the world state from tai.
        //let world_state = WorldState::world_at_tai(tai);
        //debug!("t: {tai},   triangle_rotate: {:?} deg", Deg::<f32>::from(triangle_rotate));

        let viewport = Self::viewport_from_paint_info(paint_callback_info);

        self.camera.set_viewport(viewport);

        //let triangle_rotate = radians(0.0_f32
        //    + tai.to_utc().timestamp_subsec_millis() as f32/1000.0
        //);

        let render_target =
            self.rendertarget_from_paint_info(paint_callback_info, egui_glow_painter);

        let scissor_box = Self::scissor_box_from_paint_info(paint_callback_info);

        // Clear depth 1.0 is the far plane.
        //render_target.clear_partially(scissor_box, ClearState::depth(1.0));
        render_target.clear(ClearState::depth(1.0));

        // /*
        if let Some(object) = self.opt_object_triangle.as_mut() {
            object.set_transformation(Mat4::from_angle_y(tri_rot_y));

            render_target.render_partially(scissor_box, &self.camera, [&object], &[]);
        }
        // */

        // /*
        if let Some(object) = self.opt_gm_mesh_color.as_mut() {
            object.set_transformation(Mat4::from_angle_y(tri_rot_y));

            render_target.render_partially(scissor_box, &self.camera, [&object], &[]);
        }
        // */
        
        //render_target.clear(ClearState::depth(1.0));

        // Take back the screen fbo, we will continue to use it.
        //render_target.into_framebuffer() //? TODO use this
    }
}
