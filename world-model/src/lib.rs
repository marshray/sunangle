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
//? use log::{debug, error, info, trace, warn};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
use petgraph::Graph;
//? use serde::{Deserialize, Serialize};

use coordinate_systems::*;

mod earth_agnostic;
mod earth_ellipsoid;

pub struct WorldModel {}
impl WorldModel {
    pub fn new() -> WorldModel {
        let mut deps = Graph::<&str, &str>::new();//? petgraph example code
        let pg = deps.add_node("petgraph");//? petgraph example code
        let fb = deps.add_node("fixedbitset");//? petgraph example code
        let qc = deps.add_node("quickcheck");//? petgraph example code
        let rand = deps.add_node("rand");//? petgraph example code
        let libc = deps.add_node("libc");//? petgraph example code
        deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);//? petgraph example code
        WorldModel {}
    }
}

//pub trait DefinesOrigin3d {}
//pub trait DefinesOrientation3d {}



/// WGS 84 geodetic lat, lon (degree), height (m above ellipsoid)
/// as defined by
/// - [EPSG:9754](https://spatialreference.org/ref/epsg/9754/) WGS 84 (G2139)
/// - [EPSG:4979](https://spatialreference.org/ref/epsg/4979/) WGS 84
///   - "Used by the GPS satellite navigation system."
///   - "Aug. 27, 2007"
///   - "World (by country)"
fn earth_plh() {
    /* let s_epsg_4979 = r##"
        GEOGCS["WGS 84",
            DATUM["World Geodetic System 1984",
                SPHEROID["WGS 84",6378137.0,298.257223563,
                    AUTHORITY["EPSG","7030"]],
                AUTHORITY["EPSG","6326"]],
            PRIMEM["Greenwich",0.0,
                AUTHORITY["EPSG","8901"]],
            UNIT["degree",0.017453292519943295],
            AXIS["Geodetic latitude",NORTH],
            AXIS["Geodetic longitude",EAST],
            AXIS["Ellipsoidal height",UP],
            AUTHORITY["EPSG","4979"]]"##; */
   
    let s_epsg_9754 = r##"
        GEOGCRS["WGS 84 (G2139)",
        DYNAMIC[
            FRAMEEPOCH[2016]],
        DATUM["World Geodetic System 1984 (G2139)",
            ELLIPSOID["WGS 84",6378137,298.257223563,
                LENGTHUNIT["metre",1]]],
        PRIMEM["Greenwich",0,
            ANGLEUNIT["degree",0.0174532925199433]],
        CS[ellipsoidal,3],
            AXIS["geodetic latitude (Lat)",north,
                ORDER[1],
                ANGLEUNIT["degree",0.0174532925199433]],
            AXIS["geodetic longitude (Lon)",east,
                ORDER[2],
                ANGLEUNIT["degree",0.0174532925199433]],
            AXIS["ellipsoidal height (h)",up,
                ORDER[3],
                LENGTHUNIT["metre",1]],
        USAGE[
            SCOPE["Geodesy. Navigation and positioning using GPS satellite system."],
            AREA["World: Afghanistan, Albania, [...elided...], Zambia, Zimbabwe."],
            BBOX[-90,-180,90,180]],
        ID["EPSG",9754]]"##;


}

/// [EPSG 7030](https://epsg.io/7030-ellipsoid)

/// # Notes
/// 
/// [IERS Reference Meridian](https://en.wikipedia.org/wiki/IERS_Reference_Meridian)
/// [WGS 84](https://en.wikipedia.org/wiki/World_Geodetic_System#WGS84)
///
/// "WGS 84 has most recently been updated to use the reference frame G2139, which was released
/// on January 3, 2021"
/// (https://en.wikipedia.org/wiki/World_Geodetic_System#Updates_and_new_standards)
///  reference frame G2139
///
///     Identifiers
///     Components of WGS 84 are identified by codes in the EPSG Geodetic Parameter Dataset:[18]
///         EPSG:4326 – 2D coordinate reference system (CRS)
///         EPSG:4979 – 3D CRS
///         EPSG:4978 – geocentric 3D CRS
///         EPSG:7030 – reference ellipsoid
///         EPSG:6326 – horizontal datum"
/// 
/// "Most geographic information systems (GIS) and GIS libraries use EPSG codes as Spatial Reference
/// System Identifiers (SRIDs) and EPSG definition data for identifying coordinate reference
/// systems, projections, and performing transformations between these systems, while some also
/// support SRIDs issued by other organizations (such as Esri)."
/// - [Wp: EPSG Geodetic Parameter Dataset](https://en.wikipedia.org/wiki/EPSG_Geodetic_Parameter_Dataset)
/// 
/// - [EPSG:4326](https://spatialreference.org/ref/epsg/4326/) WGS 84
///   - "Horizontal component of 3D system. Used by the GPS satellite navigation system and for NATO military geodetic surveying."
///   - "Aug. 27, 2007"
///   - This is just lat-lon
/// - [EPSG:4327](https://spatialreference.org/ref/epsg/4327/) WGS 84 (geographic 3D)
///   - Deprecated, use 4329 instead, use 4979 instead
/// - [EPSG:4328](https://spatialreference.org/ref/epsg/4328/) WGS 84 (geocentric)
///   - Deprecated, use 4978 instead
/// - [EPSG:4329](https://spatialreference.org/ref/epsg/4329/) WGS 84 (3D)
///   - Deprecated, use 4979 instead

/*
/// [](https://git.osgeo.org/gitea/postgis/postgis/src/commit/f36874a448e9256aa231e01d44334c1278ebf7a6/postgis/postgis.sql.in#L2121)
#[allow(non_camel_case_types)]
struct Postgis_spatial_ref_sys {
    srid: i32,
    auth_name: CowStaticStr,
    auth_srid: i32,
    srtext: CowStaticStr,
    proj4text: CowStaticStr,
}
fn epsg_4326_wgs_84() -> Postgis_spatial_ref_sys {
    // https://git.osgeo.org/gitea/postgis/postgis/src/commit/f36874a448e9256aa231e01d44334c1278ebf7a6/spatial_ref_sys.sql
    //    -- EPSG 4326 : WGS 84 
    //    --
    //    (4326, 'EPSG', 4326, 'GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4326"]]', '+proj=longlat +datum=WGS84 +no_defs '),
    Postgis_spatial_ref_sys {
        srid: 4326,
        auth_name: "EPSG".into(),
        auth_srid: 4326,
        srtext: r##"GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4326"]]"##.into(),
        proj4text: r##"+proj=longlat +datum=WGS84 +no_defs"##.into(),
    }
}
// */

/// [EPSG:10176](https://epsg.org/crs_10176/IGS20.html) IGS20
///
///    International GNSS Service (IGS), "Upcoming switch to IGS20", IGSMAIL-8238 and IGSMAIL-8282, https://igs.org/
///
///    Used for products from the International GNSS Service (IGS) from 2022-11-27. Replaces IGb14 (code 9378). For most practical purposes IGS20 is equivalent to ITRF2020.
/// 
///    Earth-centred, Earth-fixed (ECEF) right-handed Cartesian 3D CS, used in geocentric coordinate reference systems.

/// WGS 84 X, Y, Z (m)
/// as defined by
/// - [EPSG:4978 WGS 84](https://spatialreference.org/ref/epsg/4978/) WGS 84
///   - "Used by the GPS satellite navigation system."
///   - "Aug. 25, 2006"
/// 
/// - [EPSG:9753 WGS 84 (G2139)](https://crs-explorer.proj.org/wkt2/EPSG/9753.txt)
fn earth_geocentric_xyz() {
    //let mut ccs_wgs_84_g2139 = coordinate_systems::XYZ.clone();

    let s_epsg_4978 = r##"
        GEOCCS["WGS 84",
            DATUM["World Geodetic System 1984",
                SPHEROID["WGS 84",6378137.0,298.257223563,
                    AUTHORITY["EPSG","7030"]],
                AUTHORITY["EPSG","6326"]],
            PRIMEM["Greenwich",0.0,
                AUTHORITY["EPSG","8901"]],
            UNIT["m",1.0],
            AXIS["Geocentric X",OTHER],
            AXIS["Geocentric Y",EAST],
            AXIS["Geocentric Z",NORTH],
            AUTHORITY["EPSG","4978"]]"##;

   
    let s_epsg_9753 = r##"
        GEODCRS["WGS 84 (G2139)",
            DYNAMIC[
                FRAMEEPOCH[2016]],
            DATUM["World Geodetic System 1984 (G2139)",
                ELLIPSOID["WGS 84",6378137,298.257223563,
                    LENGTHUNIT["metre",1]]],
            PRIMEM["Greenwich",0,
                ANGLEUNIT["degree",0.0174532925199433]],
            CS[Cartesian,3],
                AXIS["(X)",geocentricX,
                    ORDER[1],
                    LENGTHUNIT["metre",1]],
                AXIS["(Y)",geocentricY,
                    ORDER[2],
                    LENGTHUNIT["metre",1]],
                AXIS["(Z)",geocentricZ,
                    ORDER[3],
                    LENGTHUNIT["metre",1]],
            USAGE[
                SCOPE["Geodesy. Navigation and positioning using GPS satellite system."],
                AREA["World."],
                BBOX[-90,-180,90,180]],
            ID["EPSG",9753]]"##;
         
}

