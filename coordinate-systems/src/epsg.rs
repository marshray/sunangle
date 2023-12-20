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
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum::{self, EnumProperty, EnumString};

/// These are the values used on [epsg.io](https://epsg.io/) for the web search query type filter.
/// 
/// They do not seem to be the same as used by `OGC WKT2`.
/// 
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(EnumProperty, EnumString)]
#[derive(Deserialize, Serialize)]
pub enum EntityKind {
    #[strum(props(k="",        d="Coordinate reference system"))            ] CRS,
    #[strum(props(k="CRS",     d="Projected coordinate reference system"))  ] PROJCRS,
    #[strum(props(k="CRS",     d="Geodetic coordinate reference system"))   ] GEOGCRS,
    #[strum(props(k="CRS",     d="Geodetic 3D coordinate reference system"))] GEOG3DCR,
    #[strum(props(k="CRS",     d="Geocentric coordinate reference system")) ] GCENCRS,
    #[strum(props(k="CRS",     d="Vertical coordinate reference system"))   ] VERTCRS,
    #[strum(props(k="CRS",     d="Engineering coordinate reference system"))] ENGCRS,
    #[strum(props(k="CRS",     d="Compound coordinate reference system"))   ] COMPOUND,
    #[strum(props(k="CRS",     d="Derived coordinate reference system"))    ] DRVDCRS,
    #[strum(props(k="",        d="Operation"))                              ] COORDOP,
    #[strum(props(k="COORDOP", d="Transformation operation"))               ] COPTRANS,
    #[strum(props(k="COORDOP", d="Compound operation"))                     ] COPCONOP,
    #[strum(props(k="COORDOP", d="Point Motion operation"))                 ] POIMOTOP,
    #[strum(props(k="COORDOP", d="Conversion operation"))                   ] COPCON,
    #[strum(props(k="",        d="Datum"))                                  ] DATUM,
    #[strum(props(k="DATUM",   d="Vertical datum"))                         ] VERTDAT,
    #[strum(props(k="DATUM",   d="Engineering datum"))                      ] ENGDAT,
    #[strum(props(k="DATUM",   d="Geodetic datum"))                         ] GEODDAT,
    #[strum(props(k="DATUM",   d="Dynamic Geodetic datum"))                 ] DYNGEODD,
    #[strum(props(k="DATUM",   d="Ensemble datum"))                         ] ENSEMDAT,
    #[strum(props(k="",        d="Ellipsoid"))                              ] ELLIPSOI,
    #[strum(props(k="",        d="Prime meridian"))                         ] PRIMEM,
    #[strum(props(k="",        d="Method"))                                 ] METHOD,
    #[strum(props(k="",        d="Coordinate system"))                      ] CS,
    #[strum(props(k="CS",      d="Vertical coordinate system"))             ] VERTCS,
    #[strum(props(k="CS",      d="Spherical coordinate system"))            ] SPHERCS,
    #[strum(props(k="CS",      d="Cartesian coordinate system"))            ] CARTESCS,
    #[strum(props(k="CS",      d="Ellipsoidal coordinate system"))          ] ELLIPCS,
    #[strum(props(k="CS",      d="Affine coordinate system"))               ] AFFINE,
    #[strum(props(k="CS",      d="Ordinal coordinate system"))              ] ORDINAL,
    #[strum(props(k="",        d="Axis"))                                   ] AXIS,
    #[strum(props(k="",        d="Area"))                                   ] AREA,
    #[strum(props(k="",        d="Unit"))                                   ] UNIT,
    #[strum(props(k="UNIT",    d="Angle unit"))                             ] ANGUNIT,
    #[strum(props(k="UNIT",    d="Scale unit"))                             ] SCALEUNI,
    #[strum(props(k="UNIT",    d="Length unit"))                            ] LENUNIT,
    #[strum(props(k="UNIT",    d="Time unit"))                              ] TIMEUNIT,
}

impl EntityKind {
    /// The kind of the entity kind, if any.
    pub fn opt_kind_of(self) -> Option<EntityKind> {
        self.get_str("k")
            .filter(|&s| !s.is_empty())
            .and_then(|s| EntityKind::try_from(s).ok())
    }

    /// The description of the entity kind.
    pub fn description(self) -> &'static str {
        self.get_str("d").unwrap_or_default()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn description() {
        assert_ron_snapshot!(EntityKind::AXIS.description(), @r##""Axis""##);
    }

    #[test]
    fn opt_kind_of() {
        assert_ron_snapshot!(EntityKind::CRS.opt_kind_of(), @"None");
        assert_ron_snapshot!(EntityKind::PROJCRS.opt_kind_of(), @"Some(CRS)");
        assert_ron_snapshot!(EntityKind::COORDOP.opt_kind_of(), @"None");
        assert_ron_snapshot!(EntityKind::POIMOTOP.opt_kind_of(), @"Some(COORDOP)");
    }
}


    // https://epsg.io/1309-area
    // "Asia - Malaysia (west) and Singapore"
    // "Information source: OGP"
    // "Data source: EPSG"
    // "Revision date: 2014-05-01"
    // file: urn:ogc:def:extent-polygon:EPSG::1309
    // no JSON, WGS2, etc

    // https://epsg.io/1309
    //     JSON "type": "Transformation"

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
/* 
    #[derive(
        Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
    )]
    enum EpsgEntityType {
        Crs,
        Op,
        Datum,
        DatumEnsemble,
        Ellipsoid,
        Primem,
        Method,
        Cs, // possibly Geodetic
        Axis,
        Area,
        Units,
    }
    impl EpsgEntityType {
        /// For some purposes, like url creation, 'DatumEnsemble' is converted to just 'Datum'.
        pub fn remove_ensemble(self) -> Self {
            use EpsgEntityType::*;
            if self == DatumEnsemble {
                Datum
            } else {
                self
            }
        }

        pub fn to_lowercase(self) -> String {
            self.to_string().to_lowercase()
        }

        pub fn entity_ref(self, id: u32) -> EpsgEntityRef {
            EpsgEntityRef(self, id)
        }
    }
 */
#[derive(
    Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[display("EPSG:{}", self.id_maybe_with_type())]
struct EpsgEntityRef(EpsgEntityType, u32);
impl EpsgEntityRef {
    pub fn id_maybe_with_type(&self) -> String {
        use EpsgEntityType::*;
        let epsg_ty = self.0.remove_ensemble();
        match epsg_ty {
            // These types do not seem to require a suffix
            Cs //? Crs, Op, Datum, Ellipsoid, Primem, Method, Axis, Area, Units,
                => self.1.to_string(),
            // Other types need a suffix
            _
                => format!("{}-{}", self.1, epsg_ty.to_lowercase()),
        }
    }

    pub fn url_epsg_io(&self) -> Url {
        format!("https://epsg.io/{}", self.id_maybe_with_type()).into()
    }
    pub fn url_epsg_io_json(&self) -> Url {
        self.url_epsg_io_file_ext("json")
    }
    pub fn url_epsg_io_wkt2(&self) -> Url {
        self.url_epsg_io_file_ext("wkt2")
    }

    fn url_epsg_io_file_ext<S: Into<CowStStr>>(&self, file_ext: S) -> Url {
        //fn url_epsg_io_file_ext<S: std::borrow::Borrow<str> + std::ops::Deref>(&self, file_ext: &S) -> Url {
        let file_ext = file_ext.into();
        //? debug_assert!(file_ext.matches("^[_a-zA-Z0-9]+$"));
        format!("{}.{}", self.url_epsg_io().0, file_ext).into()
    }
}

#[test]
fn t_EpsgEntityRef_url_epsg_io() {
    // https://epsg.io/4978.wkt2  GEODCRS["WGS 84",ENSEMBLE

    let er = EpsgEntityType::DatumEnsemble.entity_ref(6326);
    assert_eq!(er.url_epsg_io().to_string(), "https://epsg.io/6326-datum");
    assert_eq!(
        er.url_epsg_io_json().to_string(),
        "https://epsg.io/6326-datum.json"
    );

    let er = EpsgEntityType::CRS.entity_ref(4978);
    assert_eq!(
        er.url_epsg_io_json().to_string(),
        "https://epsg.io/4978.json"
    );

    let er = EpsgEntityType::Ellipsoid.entity_ref(7030);
    assert_eq!(
        er.url_epsg_io_json().to_string(),
        "https://epsg.io/7030-ellipsoid.json"
    );

    //epsg.io/4978.json
}
