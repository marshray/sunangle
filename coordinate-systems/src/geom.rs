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

//? TODO 2-D Euclidean space described by Cartesian coordinates
//? TODO 2-D Projective space described by Homogeneous coordinates

//? TODO 3-D Euclidean space described by Cartesian coordinates
//? TODO 3-D Projective space described by Homogeneous coordinates

//=================================================================================================|

#[derive(Clone, Debug, Display, Deref, DerefMut, From, Into)]
pub struct Radius(pub EcsNum);

//=================================================================================================|

/// [Flattening](https://en.wikipedia.org/wiki/Flattening).
#[derive(Clone, Debug, Display)]
#[allow(non_camel_case_types)]
pub enum Flattening {
    F(EcsNum),
    F_inv(EcsNum),
}

impl Flattening {
    pub fn f(&self) -> Result<EcsNum> {
        use Flattening::*;
        match self {
            F(f) => Ok(f.clone()),
            F_inv(f_inv) => f_inv.recip(),
        }
    }

    pub fn f_inv(&self) -> Result<EcsNum> {
        use Flattening::*;
        match self {
            F(f) => f.recip(),
            F_inv(f_inv) => Ok(f_inv.clone()),
        }
    }
}

//=================================================================================================|

/// An "ellipsoid of revolution". Also known as an [oblate spheroid](
/// https://en.wikipedia.org/wiki/Oblate_spheroid).
#[derive(Clone, Debug, Display)]
#[display("OblateSpheroidDef {{ a: {a}, f: {f} }}")]
pub struct OblateSpheroidDef {
    /// Equatorial radius semi-axis.
    pub a: DimensionedConstant,

    /// 'Ellipticity', 'flattening', or 'oblateness'.
    pub f: Flattening,
}

//-------------------------------------------------------------------------------------------------|

/// [`hecs::Bundle`] for an ellipsoid of revolution in the [`hecs::World`].
#[derive(Bundle, Clone, Debug, Display)]
#[display("OblateSpheroid {{ opt_name: {opt_name:?}, def: {def:?} }}")]
pub struct OblateSpheroid {
    pub opt_name: Option<Name>,
    pub def: OblateSpheroidDef,
}

//-------------------------------------------------------------------------------------------------|

#[derive(Clone, Copy, Debug)]
pub struct OblateSpheroidRef(Entity);

impl OblateSpheroidRef {
    pub fn new(e: Entity, world: &World) -> Self {
        debug_assert!(
            world.satisfies::<&OblateSpheroid>(e).unwrap_or_default(),
            "Although this newtype can't prevent the OblateSpheroidRef Entity from being removed from the World, it should probably at least start out that way."
        );
        OblateSpheroidRef(e)
    }
}

//-------------------------------------------------------------------------------------------------|

pub fn ecs_add_oblatespheroid<IN, ION>(
    world: &mut World,
    e_ns_parent: Entity,
    opt_name: ION,
    def: OblateSpheroidDef,
) -> Result<Entity>
where IN: Into<Name>, ION: Into<Option<IN>>
{
    let opt_name: Option<IN> = opt_name.into();
    let opt_name = opt_name.map(Into::<Name>::into);
    //let opt_name: Option<Name> = opt_name.into();
    let os = OblateSpheroid {
        opt_name,
        def
    };

    world
        .attach_new::<Namespace, _>(e_ns_parent, os)
        .context("ecs_add_const")
}

//=================================================================================================|

pub(crate) fn ecs_add_stuff(world: &mut World) -> Result<()> {
    let ns_geometry = ecs_ns_find_or_create(world, NamePathSpec::absolute(["geometry"]))?;

    //?

    Ok(())
}
