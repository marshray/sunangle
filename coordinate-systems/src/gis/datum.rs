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

use crate::names::Namespace;
use crate::*;

//=================================================================================================|

// The 'Frame reference epoch' for dynamic datums.

//=================================================================================================|

/// Definition of a [`Datum`].
#[derive(Clone, Debug, Display)]
pub struct DatumDef {
    //? TODO official designations, naming authority, notes, etc.
}

//-------------------------------------------------------------------------------------------------|

/// [`hecs::Bundle`] for a [`Datum`] in the [`hecs::World`].
#[derive(Bundle, Clone, Debug, Display)]
#[display("Datum {{ Name({:?}), {datum_def:?} }}", name.as_str())]
pub struct Datum {
    pub name: Name,
    pub datum_def: DatumDef,
}

//-------------------------------------------------------------------------------------------------|

/// Reference to a [`Datum`] in the [`hecs::World`].
#[derive(Clone, Copy, Debug)]
pub struct DatumRef(Entity);

impl DatumRef {
    pub fn new(e: Entity, world: &World) -> Self {
        debug_assert!(
            world.satisfies::<&Datum>(e).unwrap_or_default(),
            "Although this newtype can't prevent the Datum Entity from being removed from the World, it should probably at least start out that way."
        );
        DatumRef(e)
    }
}

//-------------------------------------------------------------------------------------------------|

fn ecs_add_datum(
    world: &mut World,
    e_ns_parent: Entity,
    name: &str,
) -> Result<Entity> {
    let name: Name = name.into();

    let datum_def = DatumDef { };

    let datum = Datum {
        name,
        datum_def,
    };
    
    world.attach_new::<Namespace, _>(e_ns_parent, datum)
        .context("ecs_add_datum({name:?})")
}

//=================================================================================================|

pub(crate) fn ecs_add_stuff(world: &mut World, ns_gis: Entity) -> Result<()> {
    use DimensionKind::*;

    let ns_root = RootNamespace::find_or_create(world)?;

    let ns_datum = world.attach_new::<Namespace, _>(ns_root, (Name::from("datum"),))?;
    
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
    let earth_datum = ecs_add_datum(world, ns_datum, "earth,WGS 84,G2139,dynamic,EPSG datum 1309")?;

    /*
    let earth_ellipsoid = ecs_add_ellipsoid(world, ns_gl, "earth,WGS84,dynamic,G2139,EPSG1309,from EPSG datum 1309")?;

    let oblate_spheroid_geodetic_cs = ecs_add_cs(world, ns_gl, "oblate spheroid")?;

    let earth_geodetic_crs = ecs_add_datum(world, ns_gl, "Earth")?;
    */

    Ok(())
}
