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

//? use anyhow::{anyhow, bail, ensure, Context, Result};
use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use bevy_ecs::prelude::*;

//=================================================================================================|

//-------------------------------------------------------------------------------------------------|
/*
#[derive(Default, Deref, DerefMut)]
struct World {
    #[deref]
    #[deref_mut]
    pub world: hecs::World,
}

impl World {
    pub fn new() -> World {
        Self::default()
    }
}

impl serde::Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ctx = SerContext;

        hecs::serialize::row::serialize(self, &mut ctx, serializer)

        //let mut s = serializer.serialize_struct("SWorld", 1)?;
        //s.serialize_field("name", &self.name)?;
        //s.end()
    }
}

//=================================================================================================|

struct SerContext;

impl hecs::serialize::row::SerializeContext for SerContext {
    fn serialize_entity<S>(&mut self, entity: hecs::EntityRef<'_>, mut map: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        Name::serialize_entity(entity, &mut map)?;
        DimensionKind::serialize_entity(entity, &mut map)?;
        Exactness::serialize_entity(entity, &mut map)?;
        Ratio::<u64>::serialize_entity(entity, &mut map)?;
        map.end()
    }
}
 */
//=================================================================================================|

//=================================================================================================|

//trait ComponentName {
//    fn component_name() -> CowStaticStr;
//}

//=================================================================================================|
/*
trait SerializeEntity<T> {
    fn serialize_entity<S: serde::ser::SerializeMap>(
        entity: hecs::EntityRef<'_>,
        map: &mut S,
    ) -> Result<(), S::Error>;
}

#[macro_export]
macro_rules! impl_serialize_entity {
    ($id:ident, $ty:ty) => {
        impl SerializeEntity<$ty> for $ty {
            fn serialize_entity<S: serde::ser::SerializeMap>(
                entity: hecs::EntityRef<'_>,
                map: &mut S,
            ) -> Result<(), S::Error> {
                if let Some(x) = entity.get::<&$ty>() {
                    map.serialize_key(stringify!($id))?;
                    map.serialize_value(&*x)?;
                }
                Ok(())
            }
        }
    };
    ($id:ident) => {
        impl SerializeEntity<$id> for $id {
            fn serialize_entity<S: serde::ser::SerializeMap>(
                entity: hecs::EntityRef<'_>,
                map: &mut S,
            ) -> Result<(), S::Error> {
                if let Some(x) = entity.get::<&$id>() {
                    map.serialize_key(stringify!($id))?;
                    map.serialize_value(&*x)?;
                }
                Ok(())
            }
        }
    };
}
*/

//=================================================================================================|

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
#[derive(derive_more::From)]
#[derive(Component)]
struct Name(
    String
);

impl std::convert::From<&str> for Name {
    fn from(s: &str) -> Name {
        Name(s.to_string())
    }
}

//=================================================================================================|

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
#[derive(derive_more::From)]
#[derive(Component)]
struct Ratio_u64(Ratio<u64>);

//============================================================ Exactness

#[derive(Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[derive(Component)]
pub enum Exactness {
    Exact,
    Approximate,
}

//============================================================ Constants

const APPROX_PI_TIMES_2_62: u64 =
    0b_1100_1001_0000_1111_1101_1010_1010_0010_0010_0001_0110_1000_1100_0010_0011_0100_u64;

const PI: (Exactness, Ratio<u64>) = (
    Exactness::Approximate,
    Ratio::new_raw(APPROX_PI_TIMES_2_62, 1_u64 << 62),
);
const PI_INV: (Exactness, Ratio<u64>) = (
    Exactness::Approximate,
    Ratio::new_raw(1_u64 << 62, APPROX_PI_TIMES_2_62),
);

const TAU: (Exactness, Ratio<u64>) = (
    Exactness::Approximate,
    Ratio::new_raw(APPROX_PI_TIMES_2_62, 1_u64 << 61),
);
const TAU_INV: (Exactness, Ratio<u64>) = (
    Exactness::Approximate,
    Ratio::new_raw(1_u64 << 61, APPROX_PI_TIMES_2_62),
);

//=================================================================================================|

fn ecs_add_consts(world: &mut World) {
    world.spawn((Name::from("pi"), PI.0, Ratio_u64::from(PI.1)));
    //ecs_add_const(world, "pi", PI);
    //ecs_add_const(world, "pi_inv", PI_INV);
    //ecs_add_const(world, "tau", TAU);
    //ecs_add_const(world, "tau_inv", TAU_INV);
}

//============================================================
/*
/// Entity may have a WKTF2 representation.
pub trait HasWktf2Text {
    /// Returns the WKTF2 text of the entity, if any.
    fn opt_wktf2_text(&self) -> Option<&CowStaticStr>;
}
*/
//============================================================ DimensionKind

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
enum DimensionKind {
    Length,
    Angle,
    Scale,
    Time,
}

//============================================================ Unit
/*
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
enum UnitDef {
    IsBaseUnit,
    ScaleToBaseUnit {
        scale: Ratio<u64>,
        base: &'static Unit,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct Unit {
    name: CowStaticStr,
    abbr: CowStaticStr,
    kind: DimensionKind,
    def: UnitDef,
    is_built_in: bool,
}

impl Unit {
    /// The [`UnitKind`].
    fn kind(&self) -> DimensionKind {
        self.kind
    }

    /// Returns the definition of the unit.
    fn def(&self) -> &UnitDef {
        if let UnitDef::ScaleToBaseUnit { base, .. } = self.def { debug_assert_eq!(base.kind(), self.kind()) }
        &self.def
    }

    /// Returns the abbreviation of the entity.
    fn abbr(&self) -> &CowStaticStr {
        &self.abbr
    }

    /// Returns that the entity definition is built into the lib.
    fn is_built_in(&self) -> bool {
        self.is_built_in
    }
}

impl HasName for Unit {
    /// Returns the name of the entity.
    fn name(&self) -> &CowStaticStr {
        &self.name
    }
}
*/
//============================================================ Unit
/*
#[macro_export]
macro_rules! define_base_unit {
    ($id:ident, $kind:ident, $name:literal, $abbr:literal) => {
        const $id: Unit = Unit {
            name: Cow::Borrowed($name),
            abbr: Cow::Borrowed($abbr),
            kind: DimensionKind::$kind,
            def: UnitDef::IsBaseUnit,
            is_built_in: true,
        };
    };
}

#[macro_export]
macro_rules! define_derived_unit {
    ($id:ident, $kind:ident, $name:literal, $abbr:literal, $num:expr, $den:expr, $base_id:ident) => {
        define_derived_unit!(
            $id,
            $kind,
            $name,
            $abbr,
            Ratio::new_raw($num, $den),
            $base_id
        );
    };
    ($id:ident, $kind:ident, $name:literal, $abbr:literal, $ratio:expr, $base_id:ident) => {
        const $id: Unit = Unit {
            name: Cow::Borrowed($name),
            abbr: Cow::Borrowed($abbr),
            kind: DimensionKind::$kind,
            def: UnitDef::ScaleToBaseUnit {
                scale: $ratio,
                base: &$base_id,
            },
            is_built_in: true,
        };
    };
}

//---------- Length
//
define_base_unit!(UNIT_METER, Length, "meter", "m");

//---------- Time
//
define_base_unit!(UNIT_SECOND, Time, "second", "s");

// hertz	Hz	frequency	s−1

//---------- Angle
//
define_base_unit!(UNIT_REVOLUTION, Angle, "revolution", "rev");

define_derived_unit!(UNIT_DEGREE, Angle, "degree", "deg", 1, 360, UNIT_REVOLUTION);

// radian[N 1]	rad	plane angle	m/m	1
define_derived_unit!(
    UNIT_RADIAN,
    Angle,
    "radian",
    "rad",
    APPROX_TAU_INV,
    UNIT_REVOLUTION
);

// steradian[N 1]	sr	solid angle	m2/m2	1
*/

//=================================================================================================|

//----------

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use anyhow::{anyhow, bail, ensure, Context, Result};
    use insta::assert_ron_snapshot;

    #[test]
    fn t_ecs() {
        let mut world = World::new();

        ecs_add_consts(&mut world);
/*
        let entity = world
            .spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }))
            .id();
        
        let entity_ref = world.entity(entity);
        let position = entity_ref.get::<Position>().unwrap();
        let velocity = entity_ref.get::<Velocity>().unwrap();        
 */                
        //? assert_ron_snapshot!(, @"");

        /*
        let mut world = World::new();

        assert_ron_snapshot!(world, @"{}");

        ecs_add_consts(&mut world);

        /* for &unit in &*COMMMON_UNITS {
            let name = Name(unit.name().to_string());

            let _e = world.spawn((name, unit));
        } */
 */      

        let mut v = Vec::<Entity>::new();
        let mut query = world.query::<(Entity, )>();
        for (e, ) in query.iter(&world) {
            v.push(e)
        }
        assert_ron_snapshot!(v, @r###""###);

        /* assert_ron_snapshot!(world, @r###"
        {
          4294967296: {
            "Name": "pi",
            "DimensionKind": Scale,
            "Exactness": Approximate,
            "RatioU64": (14488038916154245684, 4611686018427387904),
          },
          4294967297: {
            "Name": "pi_inv",
            "DimensionKind": Scale,
            "Exactness": Approximate,
            "RatioU64": (4611686018427387904, 14488038916154245684),
          },
          4294967298: {
            "Name": "tau",
            "DimensionKind": Scale,
            "Exactness": Approximate,
            "RatioU64": (14488038916154245684, 2305843009213693952),
          },
          4294967299: {
            "Name": "tau_inv",
            "DimensionKind": Scale,
            "Exactness": Approximate,
            "RatioU64": (2305843009213693952, 14488038916154245684),
          },
        }
        "###); */
    }
}
