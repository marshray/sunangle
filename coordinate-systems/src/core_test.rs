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
use std::ops::RangeInclusive;
//? use std::sync::Arc;
//? use std::time::Instant;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumProperty, EnumString};

use crate::*;
use hecs_macros::*;

struct Name;

/// todo check out https://docs.rs/mirror-mirror/latest/mirror_mirror/index.html

//==============================================

enum AngleUnit {
    Turn,
    Radian,
    Degree,
    Arcminute,
    Arcsecond,
}

enum UnitLength {
    Meter,
}

//==============================================

/// Note that this is limited range. It does not wrap.
struct LatitudeDegree(f64);
impl LatitudeDegree {
    /// The South Pole
    const MIN_INCLUSIVE: f64 = -90.0;

    /// The North Pole
    const MAX_INCLUSIVE: f64 = 90.0;
}

impl LatitudeDegree {}

/// Value naturally wraps.
struct LongitudeDegree(f64);
impl LongitudeDegree {
    /// Exclude the "anti-prime meridian (?)"
    const MIN_EXCLUSIVE: f64 = -180.0;

    /// Allow the "anti-prime meridian (?)"
    const MAX_INCLUSIVE: f64 = 180.0;

    /// Normalize the value to within the valid range.
    pub fn normalize(self) -> Self {
        //? TODO:
        self
    }
}

impl LongitudeDegree {}

//============================================== Time units
/*
/// DatetimeOrTextStartEnd::DateTime([2023-01-01, 2023-12-31])
/// DatetimeOrTextStartEnd::DateTime(["Approximately 4.5B years ago","Today"])
pub enum DatetimeOrTextStartEnd {
    DateTime([DateTime; 2]),
    Text([String; 2]),
}

//============================================== Scope, Extent, Identifier, Remark, etc.

/// "describes a geographic area over which a CRS or coordinate operation is applicable"
struct AreaDescription(String);

struct GeographicBoundingBox {
    p_ll: Latitude,
    p_ll: Longitude,
    l_ur: Latitude,
    l_ur: Longitude,
}


//============================================== Scope

/// "describes the purposes for which a CRS, Datum, DatumEnsemble, CO or bound CRS is applied."
pub struct Scope(String);

pub struct ScopeExtent {
    //scope
    //extent
}

//============================================== Extent

/// "describes the spatial and/or temporal applicability of a CRS, datum, datum ensemble, CO, or bound CRS"
pub trait SingleExtent {}

pub struct AreaDescription; //? TODO
impl SingleExtent for AreaDescription { }

pub struct GeographicBoundingBox; //? TODO
impl SingleExtent for GeographicBoundingBox { }

/// " describes a height range over which a CRS or coordinate operation is applicable"
pub struct VerticalExtent {
    length_unit: UnitLength,
    height_min: f64,
    height_max: f64,
}
impl SingleExtent for VerticalExtent { }

pub struct TemporalExtent(DatetimeOrTextStartEnd);
impl SingleExtent for TemporalExtent { }

//? TODO use that enum thingy to derive a discriminant, then make a Set with Eq compared like that.
enum SingleExtentKind {
    AreaDescription(AreaDescription),
    GeographicBoundingBox(GeographicBoundingBox),
    VerticalExtent(VerticalExtent),
    TemporalExtent(TemporalExtent),
}

/// At least one is required.
pub struct Extent {
    set_extents: Set<SingleExtent>,
    opt_area_description: Option<Dyn
}

//============================================== Usage

pub struct Usage {
    scope: Scope,
    extent: Box<&dyn SingleExtent>,
}

/// Must be at least one usage.
pub struct Usages(Vec<Usage>);

//==============================================


pub struct ScopeExtentIdentifierRemark {
    // Can be applied to a CRS, a CO, or a bound CRS
    //usage
    //identifier
    //remark
}

pub struct ScopeExtentIdentifierRemark {
    // Can be applied to a CRS, a CO, or a bound CRS
    //usage
    //identifier
    //remark
}

//============================================== Identifier

//============================================== Remark



//============================================== Datum

enum DatumKind {
    Engineering,
    Geodetic,
    Parametric,
    Temporal,
    Vertical,
    Ensemble,
}
    // https://epsg.io/1309-datum
    // "World Geodetic System 1984 (G2139)"
    // "Data source: EPSG"
    // "Information source: "Recent Update to WGS 84 Reference Frame and NGA Transition to IGS ANTEX", NGA Office of Geomatics / GNSS Division press release #21-520."
    // "Revision date: 2021-11-04"
    // ""
    // ""
    //     JSON "type": ""
    //     JSON "type": ""
    //     JSON "type": ""
    // "DYNAMIC[FRAMEEPOCH[2016]],
    //  DATUM[ "World Geodetic System 1984 (G2139)",
    //         ELLIPSOID[
    //             "WGS 84",
    //             6378137,
    //             298.257223563,
    //             LENGTHUNIT["metre",1]
    //         ],
    //         ID["EPSG",1309]
    //  ]"

//============================================== Datum

enum EntityKind {
    Datum,
}

#[derive(Bundle)]
struct Foo {
    x: i32,
    y: char,
}

pub struct Datum {
    datum_kind: DatumKind,

}

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;
    use hecs::*;

    #[test]
    fn t() {
        let mut world = World::new();

        let datum = Datum {};
        let datum_en = world.spawn((123, true, "abc"));

/*
        let a = world.spawn((123, true, "abc"));
        {
            let er = EpsgEntityType::DatumEnsemble.entity_ref(4978);
            world.spawn((
                Name::from("WGS 84 Geoid"),
                er,
                DescriptiveUrls(vec![
                    descriptive_url("EPSG 4978 (main page at epsg.io)", er.url_epsg_io()),
                    descriptive_url("WKT2 from epsg.io", er.url_epsg_io_wkt2()),
                    descriptive_url("JSON from epsg.io", er.url_epsg_io_json()),
                    descriptive_url(
                        "WKT2 from proj.org",
                        "https://crs-explorer.proj.org/wkt2/EPSG/4978.txt",
                    ),
                ]),
            ));
        }
 */


        //assert_ron_snapshot!(, @"");
    }
}
// */
