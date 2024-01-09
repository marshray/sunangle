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
use std::ops::{RangeBounds, RangeInclusive};
//? use std::sync::Arc;
//? use std::time::Instant;

use anyhow::{anyhow, bail, ensure, Context, Result};
use derive_more::Display;
use enumflags2::{bitflags, make_bitflags, BitFlags};
use hecs::{Bundle, Entity, World};
use hecs_hierarchy::{Hierarchy, HierarchyMut, HierarchyQuery};
//use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use ecs_namespace::NamespaceTag;
use crate::*;

#[derive(Debug, Display, Clone)]
pub enum UnitDef {
    BaseUnit,

    #[display("ScaleToBaseUnit {{ scale: {}, base_unit: {:?} }}", scale, base_unit)]
    ScaleToBaseUnit {
        scale: EcsNum,
        base_unit: Entity,
    },
}

#[derive(Bundle, Debug, Display, Clone)]
#[display("Unit {{ {:?}, {:?}, {dimension_kind}, {unit_def} }}", name.to_string(), abbr.to_string())]
pub struct Unit {
    pub name: Name,
    pub abbr: Abbr,
    pub dimension_kind: DimensionKind,
    pub unit_def: UnitDef,
}

impl Unit {
    pub fn look_up(world: &World, dimension_kind: DimensionKind, name: &str) -> Option<Entity> {
        #[cfg(all(debug_print, debug_assertions))]
        eprintln!("debug: Searching for Unit of {dimension_kind} named {name:?}");

        for (e, (&dk, na)) in world.query::<(&DimensionKind, &Name)>().iter() {
            #[cfg(all(debug_print, debug_assertions))]
            eprintln!("trace: Checking {e:?} {dk} {na}");

            if dk == dimension_kind && na.as_str() == name {
                #[cfg(all(debug_print, debug_assertions))]
                eprintln!("debug: {e:?} {name:?} is Unit of {dk} kind.");

                return Some(e);
            }
        }

        #[cfg(all(debug_print, debug_assertions))]
        eprintln!("WARN: Couldn't find Unit of {dimension_kind} named {name:?}.");

        None
    }
}

fn ecs_add_unit(
    world: &mut World,
    e_ns_parent: Entity,
    dimension_kind: DimensionKind,
    name: &str,
    abbr: &str,
    unit_def: UnitDef,
) -> Entity {
    let unit = Unit {
        name: Name::from(name),
        abbr: Abbr::from(abbr),
        dimension_kind,
        unit_def,
    };

    #[cfg(all(debug_print, debug_assertions))]
    let _unit = unit.clone();

    let e = world.attach_new::<NamespaceTag, _>(e_ns_parent, unit).unwrap();

    #[cfg(all(debug_print, debug_assertions))]
    eprintln!("debug: {e:?} {_unit}");

    e
}

#[rustfmt::skip]
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum WhichPrefixes {
    ExcludePosExp,
    ExcludePosExpGt3,
    IncludeDeci,
    IncludeCenti,
    IncludeHecto,
    IncludeDeca,
}

#[rustfmt::skip]
fn ecs_add_derived_unit_si_prefixes(
    world: &mut World,
    e_ns_parent: Entity,
    base_unit: Entity,
    which: BitFlags<WhichPrefixes>
) -> Result<()> {
    use WhichPrefixes::*;

    // From https://en.wikipedia.org/wiki/Metric_prefix
    let si_prefixes = [
        ( "quetta", "Q",  30 ),
        ( "ronna",  "R",  27 ),
        ( "yotta",  "Y",  24 ),
        ( "zetta",  "Z",  21 ),
        ( "exa",    "E",  18 ),
        ( "peta",   "P",  15 ),
        ( "tera",   "T",  12 ),
        ( "giga",   "G",   9 ),
        ( "mega",   "M",   6 ),
        ( "kilo",   "k",   3_i32 ),
        ( "hecto",  "h",   2 ),
        ( "deca",   "a",   1 ),
        //( "",      "",   0 ),
        ( "deci",   "d",  -1 ),
        ( "centi",  "c",  -2 ),
        ( "milli",  "m",  -3 ),
        ( "micro",  "μ",  -6 ),
        ( "nano",   "n",  -9 ),
        ( "pico",   "p", -12 ),
        ( "femto",  "f", -15 ),
        ( "atto",   "a", -18 ),
        ( "zepto",  "z", -21 ),
        ( "yocto",  "y", -24 ),
        ( "ronto",  "r", -27 ),
        ( "quecto", "q", -30 ),
    ];

    let (base_name, base_abbr, &dimension_kind, base_unit_def) =
        world.query_one_mut::<(&Name, &Abbr, &DimensionKind, &UnitDef)>(base_unit)?;

    let base_unit_def = base_unit_def.clone();
    let base_name = String::from(base_name.clone());
    let base_abbr = String::from(base_abbr.clone());

    for pfx in si_prefixes
    {
        let (pfx_name, pfx_abbr, exp10) = pfx;
        
        if      which.contains(ExcludePosExp) && 0 < exp10
           ||   which.contains(ExcludePosExpGt3) && 3 < exp10
           || ! which.contains(  IncludeDeci) && exp10 == -1
           || ! which.contains( IncludeCenti) && exp10 == -2
           || ! which.contains( IncludeHecto) && exp10 ==  2
           || ! which.contains(  IncludeDeca) && exp10 ==  1
        {
            continue;
        }

        let name = pfx_name.to_string() + &base_name;
        let abbr = pfx_abbr.to_string() + &base_abbr;

        let bits_needed = (exp10.abs()*3322 + 999)/1000;
        let fits_in_ratiou64 = bits_needed < 64;

        let scale = if fits_in_ratiou64 {
            let r = RatioU64::from_integer(10).pow(exp10);

            #[cfg(debug_assertions)]
            {
                let r_f: f64 = (*r.numer() as f64)/(*r.denom() as f64);
                let ratio = r_f/10.0_f64.powi(exp10);
                
                #[cfg(all(debug_print, debug_assertions))]
                eprintln!("{name} exp10 = {exp10}, r = {r}, r_f = {r_f}, ratio = {ratio}");

                debug_assert!((0.875..1.125).contains(&ratio));
            }

            EcsNum::RatioU64(r)
        } else {
            EcsNum::F64(10.0_f64.powi(exp10))
        };

        let unit_def = UnitDef::ScaleToBaseUnit { scale, base_unit };

        ecs_add_unit(
            world,
            e_ns_parent,
            dimension_kind,
            &name,
            &abbr,
            unit_def,
        );

        //#[cfg(debug_assertions)]
        //Unit::look_up(world, dimension_kind, &name);
    }

    Ok(())
}

pub(crate) fn ecs_add_stuff(world: &mut World) -> Result<()> {
    use DimensionKind::*;

    let ns_root = RootNamespace::find_or_create(world)?;

    let ns_units = world.attach_new::<NamespaceTag, _>(ns_root, (Name::from("units"),))?;

    // Length units
    {
        let ns_length = world.attach_new::<NamespaceTag, _>(ns_units, (Name::from("length"),))?;

        let meter = ecs_add_unit(world, ns_length, Length, "meter", "m", UnitDef::BaseUnit);

        ecs_add_derived_unit_si_prefixes(
            world,
            meter,
            meter,
            make_bitflags!(WhichPrefixes::{IncludeCenti | IncludeDeci | ExcludePosExpGt3}),
        )?;

        let millimeter =
            Unit::look_up(world, DimensionKind::Length, "millimeter").ok_or_else(|| anyhow!(""))?;

        let inch = ecs_add_unit(
            world,
            meter,
            Length,
            "inch",
            "in",
            UnitDef::ScaleToBaseUnit {
                scale: EcsNum::RatioU64(RatioU64::new_raw(254, 10)),
                base_unit: millimeter,
            },
        );

        let foot = ecs_add_unit(
            world,
            meter,
            Length,
            "foot",
            "ft",
            UnitDef::ScaleToBaseUnit {
                scale: EcsNum::RatioU64(RatioU64::from_integer(12)),
                base_unit: inch,
            },
        );

        let _mile = ecs_add_unit(
            world,
            meter,
            Length,
            "mile",
            "mi",
            UnitDef::ScaleToBaseUnit {
                scale: EcsNum::RatioU64(RatioU64::from_integer(5280)),
                base_unit: foot,
            },
        );
    }

    // Time units
    {
        let ns_time = world.attach_new::<NamespaceTag, _>(ns_units, (Name::from("time"),))?;

        let s = ecs_add_unit(world, ns_time, Time, "second", "s", UnitDef::BaseUnit);

        ecs_add_derived_unit_si_prefixes(
            world,
            ns_time,
            s,
            make_bitflags!(WhichPrefixes::{ExcludePosExp}),
        )?;
        // hertz	Hz	frequency	s−1
    }

    // Angle units
    {
        let angle = world.attach_new::<NamespaceTag, _>(ns_units, (Name::from("angle"),))?;

        let _turn = ecs_add_unit(world, angle, Angle, "turn", "tr", UnitDef::BaseUnit);

        ecs_add_unit(
            world,
            angle,
            Angle,
            "degree",
            "°",
            UnitDef::ScaleToBaseUnit {
                scale: EcsNum::RatioU64(RatioU64::new_raw(1, 360)),
                base_unit: _turn,
            },
        );

        if let Some(e_tau_inv) =
            DimensionedConstant::look_up(world, DimensionKind::Scale, "tau_inv")
        {
            ecs_add_unit(
                world,
                angle,
                Angle,
                "radian",
                "rad",
                UnitDef::ScaleToBaseUnit {
                    scale: EcsNum::Entity(e_tau_inv),
                    base_unit: _turn,
                },
            );
        }
    }

    Ok(())
}

#[cfg(test)]
#[allow(non_snake_case, clippy::all)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;

    use hecs::{Bundle, Entity, With, World};

    #[test]
    fn t0() -> anyhow::Result<()> {
        let mut world = World::default();
        let world = &mut world;

        crate::ecs_add_stuff(world);

        {
            let mut n = 0_usize;
            let q = world.query_mut::<With<(), (&Name, &DimensionKind)>>();
            for (e, _) in q {
                n += 1;
            }
        }

        //assert_ron_snapshot!(, @"");
        Ok(())
    }
}
