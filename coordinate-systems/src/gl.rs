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
//? use std::borrow::Cow;
//? use std::collections::BTreeMap;
//use std::convert::From;
//? use std::fmt::{Debug, Display};
//? use std::ops::{RangeBounds, RangeInclusive};
//? use std::sync::{Arc, RwLock};
//? use std::time::Instant;

use anyhow::{anyhow, bail, ensure, Context, Result};
use derive_more::{Deref, DerefMut, Display, From, Into};
//? use enumflags2::{bitflags, make_bitflags, BitFlags};
use hecs::{Bundle, Entity, World};
use hecs_hierarchy::{Hierarchy, HierarchyMut, HierarchyQuery};
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use crate::gis::*;
use crate::names::Namespace;
use crate::*;

//=================================================================================================|

#[derive(Clone, Debug, Display)]
pub struct CoordinateSystemDef {
    //? TODO official designations?
    //? TODO number and nature of axes
}

#[derive(Bundle, Clone, Debug, Display)]
#[display("CoordinateSystem {{ name: {:?}, def: {def:?} }}", name.as_str())]
pub struct CoordinateSystem {
    pub name: Name,
    pub def: CoordinateSystemDef,
}

#[derive(Clone, Copy, Debug)]
pub struct CoordinateSystemRef(Entity);

impl CoordinateSystemRef {
    pub fn new(e: Entity, world: &World) -> Self {
        debug_assert!(
            world.satisfies::<&CoordinateSystem>(e).unwrap_or_default(),
            "Although this newtype can't prevent the CoordinateSystem Entity from being removed from the World, it should probably at least start out that way."
        );
        CoordinateSystemRef(e)
    }
}

//=================================================================================================|

#[derive(Bundle, Clone, Debug, Display)]
#[display("CoordinateReferenceSystem {{ Name({:?}), {cs:?}, {datum:?} }}", name.as_str())]
pub struct CoordinateReferenceSystem {
    pub name: Name,
    pub cs: CoordinateSystemRef,
    pub datum: DatumRef,
}

#[derive(Clone, Copy, Debug)]
struct CoordinateReferenceSystemRef(Entity);

impl CoordinateReferenceSystemRef {
    pub fn new(e: Entity, world: &World) -> Self {
        debug_assert!(
            world.satisfies::<&CoordinateSystem>(e).unwrap_or_default(),
            "Although this newtype can't prevent the CoordinateReferenceSystem Entity from being removed from the World, it should probably at least start out that way."
        );
        CoordinateReferenceSystemRef(e)
    }
}

//=================================================================================================|

pub fn ecs_add_cs(
    world: &mut World,
    e_ns_parent: Entity,
    name: &str,
    def: CoordinateSystemDef,
) -> Result<Entity> {
    ecs_add(
        world,
        e_ns_parent,
        name,
        CoordinateSystem {
            name: name.into(),
            def,
        },
    )
}

pub fn ecs_add_crs(
    world: &mut World,
    e_ns_parent: Entity,
    name: &str,
    datum: DatumRef,
    cs: CoordinateSystemRef,
) -> Result<Entity> {
    ecs_add(
        world,
        e_ns_parent,
        name,
        CoordinateReferenceSystem {
            name: name.into(),
            datum,
            cs,
        },
    )
}

pub(crate) fn ecs_add_stuff(world: &mut World) -> Result<()> {
    use DimensionKind::*;

    let ns_gl = ecs_ns_find_or_create(world, NamePathSpec::absolute(["gl"]))?;

    // From: https://epsg.io/1309-datum
    // "World Geodetic System 1984 (G2139)"
    // "Data source: EPSG"
    // "Information source: "Recent Update to WGS 84 Reference Frame and NGA Transition to IGS ANTEX",
    // NGA Office of Geomatics / GNSS Division press release #21-520."
    // "Revision date: 2021-11-04"
    // WKT2: "DYNAMIC[FRAMEEPOCH[2016]],
    //  DATUM[ "World Geodetic System 1984 (G2139)",
    //         ELLIPSOID[
    //             "WGS 84",
    //             6378137,
    //             298.257223563,
    //             LENGTHUNIT["metre",1]
    //         ],
    //         ID["EPSG",1309]
    //  ]"

    let e_earth_wgs84_equatorial_radius: Entity = {
        let dc_earth_wgs84_equatorial_radius = DimensionedConstant {
            name: "earth_wgs84_equatorial_radius".into(),
            dimension_kind: DimensionKind::Length,
            // notes: "This is value is only 'Exact' in the sense of the definition of the WGS 84 reference shape."
            exactness: Exactness::Exact(crate::core::ExactReason::ByDefinition),
            value: EcsNum::RatioU64(RatioU64::from_integer(6378137)),
        };

        world
            .attach_new::<Namespace, _>(ns_gl, dc_earth_wgs84_equatorial_radius)
            .context("earth_wgs84_equatorial_radius")?
    };

    /*
    let _earth_wgs84_earth_ellipsoid = {

        let f_inv_n: u64 = 298257223563;
        let f_inv_d: u64 = 1000000000;
        let f_inv_ratiou64 = RatioU64::new_raw(f_inv_n, f_inv_d);
        let f_inv_ecsnum = EcsNum::RatioU64(f_inv_ratiou64);

        let obs_def = OblateSpheroidDef {
            a: e_earth_wgs84_equatorial_radius,
            f: geom::Flattening::F_inv(f_inv_ecsnum)
        };

        crate::geom::ecs_add_oblatespheroid(
            world, ns_gl, "earth,ellipsoid,WGS84,dynamic,G2139,EPSG-1309-datum",
            obs_def
        )?
    };
    */

    /*
    let oblate_spheroid_geodetic_cs = ecs_add_cs(world, ns_gl, "oblate spheroid")?;

    let earth_geodetic_crs = ecs_add_datum(world, ns_gl, "Earth")?;
    */

    //? TODO: Perspective projection (<https://en.wikipedia.org/wiki/Transformation_matrix#Perspective_projection>)

    //? TODO: normalized device coordinates (<https://en.wikipedia.org/wiki/Clip_coordinates>)
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c Normalized device coordinates (or NDC) have three dimensions, where:
    //w3c -1.0 ≤ x ≤ 1.0
    //w3c -1.0 ≤ y ≤ 1.0
    //w3c 0.0 ≤ z ≤ 1.0
    //w3c The bottom-left corner is at (-1.0, -1.0, z).
    
    //? TODO: clip coordinates (<https://en.wikipedia.org/wiki/Clip_coordinates>)
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c Clip space coordinates have four dimensions: (x, y, z, w)
    //w3c Clip space coordinates are used for the the clip position of a vertex
    //w3c (i.e. the position output of a vertex shader), and for the clip volume.
    // Vertex shaders use clip space for @builtin(position).
    
    //w3c Normalized device coordinates and clip space coordinates are related as follows:
    //w3c If point p = (p.x, p.y, p.z, p.w) is in the clip volume, then the
    //w3c NDC are (p.x ÷ p.w, p.y ÷ p.w, p.z ÷ p.w).
    
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c Framebuffer coordinates address the pixels in the framebuffer
    //w3c They have two dimensions.
    //w3c Each pixel extends 1 unit in x and y dimensions.
    //w3c The top-left corner is at (0.0, 0.0).
    //w3c x increases to the right.
    //w3c y increases down.
    // Fragment shaders use framebuffer coordinates space for @builtin(position).
    
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c Viewport coordinates combine framebuffer coordinates in x and y dimensions, with depth in z.
    //w3c Normally 0.0 ≤ z ≤ 1.0, but this can be modified by setting [[viewport]].minDepth and maxDepth via setViewport()
    
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c Fragment coordinates match viewport coordinates.
    
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c UV coordinates are used to sample textures, and have two dimensions:
    //w3c 0 ≤ u ≤ 1.0
    //w3c 0 ≤ v ≤ 1.0
    //w3c (0.0, 0.0) is in the first texel in texture memory address order.
    //w3c (1.0, 1.0) is in the last texel texture memory address order.
    
    // [](<https://gpuweb.github.io/gpuweb/#coordinate-systems>)
    //w3c Window coordinates, or present coordinates, match framebuffer coordinates, and are
    //w3c used when interacting with an external display or conceptually similar interface.

    Ok(())
}
