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
//? use serde::{Deserialize, Serialize};

use crate::*;

/// `XY` - the well-known 2D Cartesian system.
///
pub static XY: Lazy<SpecificCartesianCoordinateSystem<2>> =
    Lazy::new(|| SpecificCartesianCoordinateSystem::<2> {
        scs: SpecificCoordinateSystem::<2> {
            name: "XY".into(),
            urls: vec![
                (
                    "three_d::core::prelude::Vector3 (Rust)".into(),
                    "https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector2.html".into(),
                ),
                (
                    "cgmath::EuclideanSpace (Rust)".into(),
                    "https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html".into(),
                ),
                (
                    "CGAL::Cartesian<2> (C++)".into(),
                    "https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html".into(),
                ),
                (
                    "Three-dimensional space".into(),
                    "https://en.wikipedia.org/wiki/Plane_(mathematics)".into(),
                ),
                (
                    "Cartesian coordinate system".into(),
                    "https://en.wikipedia.org/wiki/Cartesian_coordinate_system".into(),
                ),
            ],
            structure: CSStructure::Cartesian,
            dim_infos: vec![
                DimensionInfo {
                    dim_name: "X".into(),
                    var_name: "x".into(),
                    opt_cyclic_bounds: None,
                },
                DimensionInfo {
                    dim_name: "Y".into(),
                    var_name: "y".into(),
                    opt_cyclic_bounds: None,
                },
            ],
        },
    });

/// `XYZ` - the well-known 3D Cartesian system.
pub static XYZ: Lazy<SpecificCartesianCoordinateSystem<3>> =
    Lazy::new(|| SpecificCartesianCoordinateSystem::<3> {
        scs: SpecificCoordinateSystem::<3> {
            name: "XYZ".into(),
            urls: vec![
                (
                    "three_d::core::prelude::Vector3 (Rust)".into(),
                    "https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector3.html".into(),
                ),
                (
                    "cgmath::EuclideanSpace (Rust)".into(),
                    "https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html".into(),
                ),
                (
                    "CGAL::Cartesian<3> (C++)".into(),
                    "https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html".into(),
                ),
                (
                    "Three-dimensional space".into(),
                    "https://en.wikipedia.org/wiki/Three-dimensional_space".into(),
                ),
                (
                    "Cartesian coordinate system".into(),
                    "https://en.wikipedia.org/wiki/Cartesian_coordinate_system".into(),
                ),
            ],
            structure: CSStructure::Cartesian,
            dim_infos: vec![
                DimensionInfo {
                    dim_name: "X".into(),
                    var_name: "x".into(),
                    opt_cyclic_bounds: None,
                },
                DimensionInfo {
                    dim_name: "Y".into(),
                    var_name: "y".into(),
                    opt_cyclic_bounds: None,
                },
                DimensionInfo {
                    dim_name: "Z".into(),
                    var_name: "z".into(),
                    opt_cyclic_bounds: None,
                },
            ],
        },
    });

    #[allow(non_snake_case)]
    pub static SPHERE: Lazy<SpecificCartesianCoordinateSystem<3>> =
    Lazy::new(|| SpecificCartesianCoordinateSystem::<3> {
        scs: SpecificCoordinateSystem::<3> {
            name: "SPHERE".into(),
            urls: vec![
                (
                    "three_d::core::prelude::Vector3 (Rust)".into(),
                    "https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector3.html".into(),
                ),
                (
                    "cgmath::EuclideanSpace (Rust)".into(),
                    "https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html".into(),
                ),
                (
                    "CGAL::Cartesian<3> (C++)".into(),
                    "https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html".into(),
                ),
                (
                    "Three-dimensional space".into(),
                    "https://en.wikipedia.org/wiki/Three-dimensional_space".into(),
                ),
                (
                    "Cartesian coordinate system".into(),
                    "https://en.wikipedia.org/wiki/Cartesian_coordinate_system".into(),
                ),
            ],
            structure: CSStructure::Ellipsoid,
            dim_infos: vec![
                DimensionInfo {
                    var_name: "latitude, φ".into(),
                    opt_cyclic_bounds: None,
                },
                DimensionInfo {
                    var_name: "longitude, λ".into(),
                    opt_cyclic_bounds: None,
                },
                DimensionInfo {
                    var_name: "ellipsoidal height, h".into(),
                    opt_cyclic_bounds: None,
                },
            ],
        },
    });

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use anyhow::{anyhow, bail, ensure, Context, Result};
    use insta::assert_ron_snapshot;
    use std::ops::Deref;

    use crate::*;

    #[test]
    fn XY_() -> anyhow::Result<()> {
        let XY_ = super::XY.deref();
        assert_ron_snapshot!(XY_, @r###"
        SpecificCartesianCoordinateSystem(
          scs: SpecificCoordinateSystem(
            name: "XY",
            urls: [
              ("three_d::core::prelude::Vector3 (Rust)", "https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector2.html"),
              ("cgmath::EuclideanSpace (Rust)", "https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html"),
              ("CGAL::Cartesian<2> (C++)", "https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html"),
              ("Three-dimensional space", "https://en.wikipedia.org/wiki/Plane_(mathematics)"),
              ("Cartesian coordinate system", "https://en.wikipedia.org/wiki/Cartesian_coordinate_system"),
            ],
            structure: CSStructure::Cartesian,
            dim_infos: [
              DimensionInfo(
                name: "x",
                opt_cyclic_bounds: None,
              ),
              DimensionInfo(
                name: "y",
                opt_cyclic_bounds: None,
              ),
            ],
          ),
        )
        "###);
        assert_ron_snapshot!(format!("{XY_:?}"), @r###""SpecificCartesianCoordinateSystem<2> { name: \"XY\", urls: [(\"three_d::core::prelude::Vector3 (Rust)\", \"https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector2.html\"), (\"cgmath::EuclideanSpace (Rust)\", \"https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html\"), (\"CGAL::Cartesian<2> (C++)\", \"https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html\"), (\"Three-dimensional space\", \"https://en.wikipedia.org/wiki/Plane_(mathematics)\"), (\"Cartesian coordinate system\", \"https://en.wikipedia.org/wiki/Cartesian_coordinate_system\")], cartesian: true, dim_infos: [DimensionInfo { name: \"x\", opt_cyclic_bounds: None }, DimensionInfo { name: \"y\", opt_cyclic_bounds: None }] }""###);
        assert_ron_snapshot!(format!("{XY_}"), @r###""SpecificCartesianCoordinateSystem<2> { name: \"XY\", urls: [(\"three_d::core::prelude::Vector3 (Rust)\", \"https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector2.html\"), (\"cgmath::EuclideanSpace (Rust)\", \"https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html\"), (\"CGAL::Cartesian<2> (C++)\", \"https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html\"), (\"Three-dimensional space\", \"https://en.wikipedia.org/wiki/Plane_(mathematics)\"), (\"Cartesian coordinate system\", \"https://en.wikipedia.org/wiki/Cartesian_coordinate_system\")], cartesian: true, dim_infos: [DimensionInfo { name: \"x\", opt_cyclic_bounds: None }, DimensionInfo { name: \"y\", opt_cyclic_bounds: None }] }""###);
        assert_ron_snapshot!(format!("{XY_:#}"), @r###""SpecificCartesianCoordinateSystem<2> {\n    name: \"XY\",\n    urls: [\n        (\n            \"three_d::core::prelude::Vector3 (Rust)\",\n            \"https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector2.html\",\n        ),\n        (\n            \"cgmath::EuclideanSpace (Rust)\",\n            \"https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html\",\n        ),\n        (\n            \"CGAL::Cartesian<2> (C++)\",\n            \"https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html\",\n        ),\n        (\n            \"Three-dimensional space\",\n            \"https://en.wikipedia.org/wiki/Plane_(mathematics)\",\n        ),\n        (\n            \"Cartesian coordinate system\",\n            \"https://en.wikipedia.org/wiki/Cartesian_coordinate_system\",\n        ),\n    ],\n    cartesian: true,\n    dim_infos: [\n        DimensionInfo {\n            name: \"x\",\n            opt_cyclic_bounds: None,\n        },\n        DimensionInfo {\n            name: \"y\",\n            opt_cyclic_bounds: None,\n        },\n    ],\n}""###);
        assert_ron_snapshot!(XY_.name(), @r###""XY""###);
        assert_ron_snapshot!(XY_.cs_structure(), @"");
        assert_ron_snapshot!(XY_.cnt_dimensions(), @"2");
        assert_ron_snapshot!(XY_.dimension_info(0), @r###"
        Some(DimensionInfo(
          name: "x",
          opt_cyclic_bounds: None,
        ))
        "###);
        assert_ron_snapshot!(XY_.dimension_info(1), @r###"
        Some(DimensionInfo(
          name: "y",
          opt_cyclic_bounds: None,
        ))
        "###);
        assert_ron_snapshot!(XY_.dimension_info(2), @"None");
        assert_ron_snapshot!(XY_.origin(), @"(0.0, 0.0)");
        Ok(())
    }

    #[test]
    fn XYZ_() -> anyhow::Result<()> {
        let XYZ_ = super::XYZ.deref();
        assert_ron_snapshot!(XYZ_, @r###"
        SpecificCartesianCoordinateSystem(
          scs: SpecificCoordinateSystem(
            name: "XYZ",
            urls: [
              ("three_d::core::prelude::Vector3 (Rust)", "https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector3.html"),
              ("cgmath::EuclideanSpace (Rust)", "https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html"),
              ("CGAL::Cartesian<3> (C++)", "https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html"),
              ("Three-dimensional space", "https://en.wikipedia.org/wiki/Three-dimensional_space"),
              ("Cartesian coordinate system", "https://en.wikipedia.org/wiki/Cartesian_coordinate_system"),
            ],
            structure: CSStructure::Cartesian,
            dim_infos: [
              DimensionInfo(
                name: "x",
                opt_cyclic_bounds: None,
              ),
              DimensionInfo(
                name: "y",
                opt_cyclic_bounds: None,
              ),
              DimensionInfo(
                name: "z",
                opt_cyclic_bounds: None,
              ),
            ],
          ),
        )
        "###);
        assert_ron_snapshot!(format!("{XYZ_:?}"), @r###""SpecificCartesianCoordinateSystem<3> { name: \"XYZ\", urls: [(\"three_d::core::prelude::Vector3 (Rust)\", \"https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector3.html\"), (\"cgmath::EuclideanSpace (Rust)\", \"https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html\"), (\"CGAL::Cartesian<3> (C++)\", \"https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html\"), (\"Three-dimensional space\", \"https://en.wikipedia.org/wiki/Three-dimensional_space\"), (\"Cartesian coordinate system\", \"https://en.wikipedia.org/wiki/Cartesian_coordinate_system\")], cartesian: true, dim_infos: [DimensionInfo { name: \"x\", opt_cyclic_bounds: None }, DimensionInfo { name: \"y\", opt_cyclic_bounds: None }, DimensionInfo { name: \"z\", opt_cyclic_bounds: None }] }""###);
        assert_ron_snapshot!(format!("{XYZ_}"), @r###""SpecificCartesianCoordinateSystem<3> { name: \"XYZ\", urls: [(\"three_d::core::prelude::Vector3 (Rust)\", \"https://docs.rs/three-d/0.16.3/three_d/core/prelude/struct.Vector3.html\"), (\"cgmath::EuclideanSpace (Rust)\", \"https://docs.rs/cgmath/0.18.0/cgmath/trait.EuclideanSpace.html\"), (\"CGAL::Cartesian<3> (C++)\", \"https://doc.cgal.org/5.6/Kernel_23/structCGAL_1_1Cartesian.html\"), (\"Three-dimensional space\", \"https://en.wikipedia.org/wiki/Three-dimensional_space\"), (\"Cartesian coordinate system\", \"https://en.wikipedia.org/wiki/Cartesian_coordinate_system\")], cartesian: true, dim_infos: [DimensionInfo { name: \"x\", opt_cyclic_bounds: None }, DimensionInfo { name: \"y\", opt_cyclic_bounds: None }, DimensionInfo { name: \"z\", opt_cyclic_bounds: None }] }""###);
        assert_ron_snapshot!(XYZ_.name(), @r###""XYZ""###);
        assert_ron_snapshot!(XYZ_.structure(), @"");
        assert_ron_snapshot!(XYZ_.cnt_dimensions(), @"3");
        assert_ron_snapshot!(XYZ_.dimension_info(0), @r###"
        Some(DimensionInfo(
          name: "x",
          opt_cyclic_bounds: None,
        ))
        "###);
        assert_ron_snapshot!(XYZ_.dimension_info(1), @r###"
        Some(DimensionInfo(
          name: "y",
          opt_cyclic_bounds: None,
        ))
        "###);
        assert_ron_snapshot!(XYZ_.dimension_info(2), @r###"
        Some(DimensionInfo(
          name: "z",
          opt_cyclic_bounds: None,
        ))
        "###);
        assert_ron_snapshot!(XYZ_.dimension_info(3), @"None");
        assert_ron_snapshot!(XYZ_.origin(), @"(0.0, 0.0, 0.0)");
        Ok(())
    }
}


pub static COORDINATE_SYSTEMS: Lazy<Vec<&'static (dyn CoordinateSystem + Send + Sync)>> =
    Lazy::new(|| vec![XY.deref(), XYZ.deref()]);

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use anyhow::{anyhow, bail, ensure, Context, Result};
    use insta::assert_ron_snapshot;

    #[test]
    fn t() -> anyhow::Result<()> {
        let coordinate_systems = COORDINATE_SYSTEMS.deref();
        let coordinate_system_names = coordinate_systems
            .iter()
            .map(|&cs| cs.name())
            .collect::<Vec<_>>();
        assert_ron_snapshot!(coordinate_system_names, @r###"
        [
          "XY",
          "XYZ",
        ]
        "###);
        Ok(())
    }
}
