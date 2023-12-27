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
use std::borrow::Cow;
//? use std::fmt::{Debug, Display};
//? use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_integer::Integer;
use num_rational::Ratio;
//? use num_traits::identities::Zero;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
//? use strum::{self, EnumProperty, EnumString};

use crate::*;

//============================================================

/// Entity has a name.
pub trait Name {
    /// Returns the name of the entity.
    fn name(&self) -> &CowStaticStr;
}

/// Entity has a WKTF2 representation.
pub trait Wktf2Text {
    /// Returns the WKTF2 text of the entity.
    fn wktf2_text(&self) -> &CowStaticStr;
}

//============================================================ Constants

pub const APPROX_PI_TIMES_2_62: u64 =
    0b_1100_1001_0000_1111_1101_1010_1010_0010_0010_0001_0110_1000_1100_0010_0011_0100_u64;
pub const APPROX_PI: Ratio<u64> = Ratio::new_raw(APPROX_PI_TIMES_2_62, 1_u64 << 62);
pub const APPROX_TAU: Ratio<u64> = Ratio::new_raw(APPROX_PI_TIMES_2_62, 1_u64 << 61);
pub const APPROX_TAU_INV: Ratio<u64> = Ratio::new_raw(1_u64 << 61, APPROX_PI_TIMES_2_62);

//============================================================ DimensionKind

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum DimensionKind {
    Length,
    Angle,
    Scale,
    Time,
}

//============================================================ Unit

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum UnitDef {
    IsBaseUnit,
    ScaleToBaseUnit {
        scale: Ratio<u64>,
        base: &'static Unit,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Unit {
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

impl Name for Unit {
    /// Returns the name of the entity.
    fn name(&self) -> &CowStaticStr {
        &self.name
    }
}

//============================================================ Unit
/*
   pub const UNIT_METER: Unit = Unit {
       name: Cow::Borrowed("meter"),
       abbr: Cow::Borrowed("m"),
       kind: DimensionKind::Length,
       def: UnitDef::IsBaseUnit,
       is_built_in: true,
   };

   pub const UNIT_CENTIMETER: Unit = Unit {
       name: Cow::Borrowed("centimeter"),
       abbr: Cow::Borrowed("cm"),
       kind: DimensionKind::Length,
       def: UnitDef::ScaleToBaseUnit {
           scale: Ratio::new_raw(1, 10),
           base: &UNIT_METER,
       },
       is_built_in: true,
   };

   pub const UNIT_KILOMETER: Unit = Unit {
       name: Cow::Borrowed("kiloimeter"),
       abbr: Cow::Borrowed("km"),
       kind: DimensionKind::Length,
       def: UnitDef::ScaleToBaseUnit {
           scale: Ratio::new_raw(1000, 1),
           base: &UNIT_METER,
       },
       is_built_in: true,
   };

   pub const UNIT_SECOND: Unit = Unit {
       name: Cow::Borrowed("second"),
       abbr: Cow::Borrowed("s"),
       kind: DimensionKind::Time,
       def: UnitDef::IsBaseUnit,
       is_built_in: true,
   };
*/

#[macro_export]
macro_rules! define_base_unit {
    ($id:ident, $kind:ident, $name:literal, $abbr:literal) => {
        pub const $id: Unit = Unit {
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
        pub const $id: Unit = Unit {
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

#[macro_export]
macro_rules! define_si_prefixes {
    ($id:ident, $kind:ident, $name:literal, $abbr:literal, $num:expr, $den:expr, $base_id:ident) => {
        //define_derived_unit!($id, $kind, $name, $abbr, Ratio::new_raw($num, $den), $base_id);
        // ...
    };
}

//---------- Length
//
define_base_unit!(UNIT_METER, Length, "meter", "m");

//---------- Time
//
define_base_unit!(UNIT_SECOND, Time, "second", "s");

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

//---------- Misc
//
// hertz	Hz	frequency	s−1
// newton	N	force, weight	kg⋅m⋅s−2
// pascal	Pa	pressure, stress	kg⋅m−1⋅s−2	N/m2 = J/m3
// joule	J	energy, work, heat	kg⋅m2⋅s−2	N⋅m = Pa⋅m3
// watt	W	power, radiant flux	kg⋅m2⋅s−3	J/s
// coulomb	C	electric charge	s⋅A
// volt	V	electric potential, voltage, emf	kg⋅m2⋅s−3⋅A−1	W/A = J/C
// farad	F	capacitance	kg−1⋅m−2⋅s4⋅A2	C/V = C2/J
// ohm	Ω	resistance, impedance, reactance	kg⋅m2⋅s−3⋅A−2	V/A = J⋅s/C2
// siemens	S	electrical conductance	kg−1⋅m−2⋅s3⋅A2	Ω−1
// weber	Wb	magnetic flux	kg⋅m2⋅s−2⋅A−1	V⋅s
// tesla	T	magnetic flux density	kg⋅s−2⋅A−1	Wb/m2
// henry	H	inductance	kg⋅m2⋅s−2⋅A−2	Wb/A
// degree Celsius	°C	temperature relative to 273.15 K	K
// lumen	lm	luminous flux	cd⋅m2/m2	cd⋅sr
// lux	lx	illuminance	cd⋅m2/m4	lm/m2 = cd⋅sr⋅m−2
// becquerel	Bq	activity referred to a radionuclide (decays per unit time)	s−1
// gray	Gy	absorbed dose (of ionising radiation)	m2⋅s−2	J/kg
// sievert	Sv	equivalent dose (of ionising radiation)	m2⋅s−2	J/kg
// katal	kat	catalytic activity	mol⋅s−1

#[rustfmt::skip]
pub static COMMMON_UNITS: Lazy<Vec<&'static Unit>> = Lazy::new(|| vec![
    &UNIT_METER,
    //&UNIT_CENTIMETER,
    //&UNIT_KILOMETER,
    &UNIT_SECOND,
    &UNIT_REVOLUTION,
]);

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use anyhow::{anyhow, bail, ensure, Context, Result};
    use insta::assert_ron_snapshot;

    #[test]
    fn t() {
        assert_ron_snapshot!(*COMMMON_UNITS, @r###"
        [
          Unit(
            name: "meter",
            abbr: "m",
            kind: Length,
            def: IsBaseUnit,
            is_built_in: true,
          ),
          Unit(
            name: "second",
            abbr: "s",
            kind: Time,
            def: IsBaseUnit,
            is_built_in: true,
          ),
          Unit(
            name: "revolution",
            abbr: "rev",
            kind: Angle,
            def: IsBaseUnit,
            is_built_in: true,
          ),
        ]
        "###);
    }
}
