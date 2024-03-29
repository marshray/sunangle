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
//? use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

use anyhow::{anyhow, bail, ensure, Context, Result};
use derive_more::Display;
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

use ecs_namespace::NamespaceTag;
use crate::*;

//=================================================================================================|

#[derive(Bundle, Clone, Debug, Display)]
#[display("DimensionedConstant {{ {:?}, {dimension_kind}, {exactness}, {value} }}", name.to_string())]
pub struct DimensionedConstant {
    pub name: Name,
    pub dimension_kind: DimensionKind,
    pub exactness: Exactness,
    pub value: EcsNum,
}

impl DimensionedConstant {
    pub fn look_up(world: &World, dimension_kind: DimensionKind, name: &str) -> Option<Entity> {
        //#[cfg(debug_assertions)] eprintln!("debug: Searching for DimensionedConstant of {dimension_kind} named {name:?}");

        for (e, (&dk, na)) in world.query::<(&DimensionKind, &Name)>().iter() {
            //#[cfg(debug_assertions)] eprintln!("trace: Checking {e:?} {dk} {na}");

            if dk == dimension_kind && na.as_str() == name {
                #[cfg(debug_assertions)]
                eprintln!("debug: {e:?} {name:?} is DimensionedConstant of {dk} kind.");
                return Some(e);
            }
        }

        #[cfg(debug_assertions)]
        eprintln!("WARN: Couldn't find DimensionedConstant of {dimension_kind} named {name:?}.");

        None
    }
}

//=================================================================================================|

const APPROX_PI_TIMES_2_62: u64 =
    0b_1100_1001_0000_1111_1101_1010_1010_0010_0010_0001_0110_1000_1100_0010_0011_0100_u64;

//=================================================================================================|

fn ecs_add_const(
    world: &mut World,
    e_ns_parent: Entity,
    name: &str,
    numer: u64,
    denom: u64,
) -> Result<Entity> {
    let dc = DimensionedConstant {
        name: Name(String::from(name)),
        dimension_kind: DimensionKind::Scale,
        exactness: Exactness::Approximate,
        value: EcsNum::RatioU64(RatioU64::new_raw(numer, denom)),
    };

    world
        .attach_new::<NamespaceTag, _>(e_ns_parent, dc)
        .context("ecs_add_const")
}

pub(crate) fn ecs_add_stuff(world: &mut World) -> Result<()> {
    let ns_root = RootNamespace::find_or_create(world)?;

    let ns_consts = world.attach_new::<NamespaceTag, _>(ns_root, (Name::from("consts"),))?;

    ecs_add_const(world, ns_consts, "pi", APPROX_PI_TIMES_2_62, 1_u64 << 62)?;
    ecs_add_const(
        world,
        ns_consts,
        "pi_inv",
        1_u64 << 62,
        APPROX_PI_TIMES_2_62,
    )?;
    ecs_add_const(world, ns_consts, "tau", APPROX_PI_TIMES_2_62, 1_u64 << 61)?;
    ecs_add_const(
        world,
        ns_consts,
        "tau_inv",
        1_u64 << 61,
        APPROX_PI_TIMES_2_62,
    )?;

    Ok(())
}
