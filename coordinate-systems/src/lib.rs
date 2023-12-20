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
use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub type CowStStr = Cow<'static, str>;

pub mod epsg;
pub mod gml;

/// Per ISO 19111:2007, a `CoordinateSystem` is a "set of mathematical rules for specifying how
/// coordinates are to be assigned to points".
/// 
struct CoordinateSystem;

/// Per ISO 19111:2007, a `CoordinateReferenceSystem` is a [`CoordinateSystem`] "that is related
/// to an object by a" [`Datum`].
/// 
/// - see OGC GML
struct CoordinateReferenceSystem;

/// Per ISO 19111:2007, a [`Datum`] is a "parameter or set of parameters that define the position
/// of the origin, the scale, and the orientation of a coordinate system".
/// Per OGC, a [`Datum`] is one of geodetic, vertical, engineering, image, or temporal.
struct Datum;

/// Per ISO 19111:2007, a [`GeodeticDatum`] is a "datum describing the relationship of a 2- or
/// 3-dimensional coordinate system to the Earth"
struct GeodeticDatum;

#[derive(
    Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
struct Name(CowStStr);
impl<S> std::convert::From<S> for Name
where
    S: Into<CowStStr>,
{
    fn from(s: S) -> Self {
        Name(s.into())
    }
}

fn name<T: Into<Name>>(val: T) -> Name {
    val.into()
}

#[derive(
    Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
struct Url(CowStStr);
impl<S> std::convert::From<S> for Url
where
    S: Into<CowStStr>,
{
    fn from(s: S) -> Self {
        let s = s.into();
        Url(s)
    }
}

fn url<T: Into<Url>>(val: T) -> Url {
    val.into()
}

struct DescriptiveUrl(CowStStr, Url);
fn descriptive_url<S: Into<CowStStr>, U: Into<Url>>(s: S, u: U) -> DescriptiveUrl {
    DescriptiveUrl(s.into(), u.into())
}

struct DescriptiveUrls(Vec<DescriptiveUrl>);

struct Wktf2Text(CowStStr);
impl<T> std::convert::From<T> for Wktf2Text
where
    T: Into<CowStStr>,
{
    fn from(val: T) -> Self {
        Wktf2Text(val.into())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;

    use hecs::*;

    #[test]
    fn t_hecs_example_transform_hierarchy() -> anyhow::Result<()> {
        // In practice this would usually also include rotation, or even be a general homogeneous matrix //? hecs example code
        #[derive(Debug, Copy, Clone, Default, Eq, PartialEq)] //? hecs example code
        struct Transform(i32, i32);
        impl std::ops::Mul for Transform {
            //? hecs example code
            type Output = Transform; //? hecs example code
            fn mul(self, rhs: Self) -> Transform {
                //? hecs example code
                Transform(self.0 + rhs.0, self.1 + rhs.1) //? hecs example code
            }
        }

        // Component of entities that are positioned relative to a parent entity //? hecs example code
        struct Parent {
            //? hecs example code
            /// Parent entity //? hecs example code
            entity: Entity, //? hecs example code
            /// Converts child-relative coordinates to parent-relative coordinates //? hecs example code
            from_child: Transform, //? hecs example code
        }

        /// Update absolute transforms based on relative transforms //? hecs example code
        fn evaluate_relative_transforms(world: &mut World) {
            //? hecs example code
            // Construct a view for efficient random access into the set of all entities that have //? hecs example code
            // parents. Views allow work like dynamic borrow checking or component storage look-up to be //? hecs example code
            // done once rather than per-entity as in `World::get`. //? hecs example code
            let mut parents = world.query::<&Parent>();
            let parents = parents.view();

            // View of entities that don't have parents, i.e. roots of the transform hierarchy //? hecs example code
            let mut roots = world.query::<&Transform>().without::<&Parent>();
            let roots = roots.view();

            // This query can coexist with the `roots` view without illegal aliasing of `Transform` //? hecs example code
            // references because the inclusion of `&Parent` in the query, and its exclusion from the view, //? hecs example code
            // guarantees that they will never overlap. Similarly, it can coexist with `parents` because //? hecs example code
            // that view does not reference `Transform`s at all. //? hecs example code
            for (_entity, (parent, absolute)) in world.query::<(&Parent, &mut Transform)>().iter() {
                //? hecs example code
                // Walk the hierarchy from this entity to the root, accumulating the entity's absolute //? hecs example code
                // transform. This does a small amount of redundant work for intermediate levels of deeper //? hecs example code
                // hierarchies, but unlike a top-down traversal, avoids tracking entity child lists and is //? hecs example code
                // cache-friendly. //? hecs example code
                let mut relative = parent.from_child; //? hecs example code
                let mut ancestor = parent.entity; //? hecs example code
                while let Some(next) = parents.get(ancestor) {
                    //? hecs example code
                    relative = next.from_child * relative; //? hecs example code
                    ancestor = next.entity; //? hecs example code
                }
                // The `while` loop terminates when `ancestor` cannot be found in `parents`, i.e. when it //? hecs example code
                // does not have a `Parent` component, and is therefore necessarily a root. //? hecs example code
                *absolute = *roots.get(ancestor).unwrap() * relative; //? hecs example code
            }
        }

        let mut world = World::new();

        // Spawn entities with no parent //? hecs example code
        let root = world.spawn((Transform(3, 4),));
        let _other_root = world.spawn((Transform(1, 2),));

        // Spawn some child entities, including dummy transform components that will later be //? hecs example code
        // overwritten with derived absolute transforms //? hecs example code
        let child = world.spawn((
            //? hecs example code
            Parent {
                //? hecs example code
                entity: root,                //? hecs example code
                from_child: Transform(1, 1), //? hecs example code
            }, //? hecs example code
            Transform::default(), //? hecs example code
        ));
        let _other_child = world.spawn((
            //? hecs example code
            Parent {
                //? hecs example code
                entity: root,                //? hecs example code
                from_child: Transform(0, 0), //? hecs example code
            }, //? hecs example code
            Transform::default(), //? hecs example code
        ));
        let grandchild = world.spawn((
            //? hecs example code
            Parent {
                //? hecs example code
                entity: child,                //? hecs example code
                from_child: Transform(-1, 0), //? hecs example code
            }, //? hecs example code
            Transform::default(), //? hecs example code
        ));

        evaluate_relative_transforms(&mut world);

        // Child entities' transforms are derived recursively from their relationship to their parent //? hecs example code
        assert_eq!(*world.get::<&Transform>(child).unwrap(), Transform(4, 5));
        assert_eq!(
            //? hecs example code
            *world.get::<&Transform>(grandchild).unwrap(), //? hecs example code
            Transform(3, 5)                                //? hecs example code
        );

        // Moving a parent and re-evaluating moves its children //? hecs example code
        *world.get::<&mut Transform>(root).unwrap() = Transform(2, 2);
        evaluate_relative_transforms(&mut world);
        assert_eq!(*world.get::<&Transform>(child).unwrap(), Transform(3, 3));
        assert_eq!(
            //? hecs example code
            *world.get::<&Transform>(grandchild).unwrap(), //? hecs example code
            Transform(2, 3)                                //? hecs example code
        );

        Ok(())
    }

    #[test]
    fn t2() -> anyhow::Result<()> {
        let mut world = World::new();

        // Nearly any type can be used as a component with zero boilerplate //? hecs example code
        let a = world.spawn((123, true, "abc"));
        let b = world.spawn((42, false));

        // Systems can be simple for loops //? hecs example code
        for (id, (number, &flag)) in world.query_mut::<(&mut i32, &bool)>() {
            //? hecs example code
            if flag {
                *number *= 2;
            }
        }

        // Random access is simple and safe //? hecs example code
        assert_eq!(*world.get::<&i32>(a).unwrap(), 246);
        assert_eq!(*world.get::<&i32>(b).unwrap(), 42);

        //assert_ron_snapshot!(, @"");
        Ok(())
    }


    #[allow(non_camel_case_types)]
    struct Ellipsoid_a_1f();

    enum ExternalEntityRef {
        Epsg(EpsgEntityRef),
    }

    /// Returns the frame reference epoch.
    /// E.g. "2015.00" for [EPSG:1333](https://epsg.org/datum_1333/IGS20.html).
    struct FrameReferenceEpoch(CowStStr);

    struct Accuracy(f64);

    // pub urls: Vec<(CowStStr, CowStStr)>,
    // pub structure: CSStructure,
    // pub dim_infos: Vec<DimensionInfo>,

    #[test]
    fn t3() -> anyhow::Result<()> {
        let mut world = World::new();

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

        {
            let er = EpsgEntityType::Ellipsoid.entity_ref(7030);

            world.spawn((
                Name::from("WGS 84 Geoid"),
                er,
                DescriptiveUrls(vec![
                    descriptive_url(
                        "EPSG (ellipsoid) 7030 (main page at epsg.io)",
                        er.url_epsg_io(),
                    ),
                    descriptive_url("WKT2 (epsg.io)", er.url_epsg_io_wkt2()),
                    descriptive_url("JSON (epsg.io)", er.url_epsg_io_json()),
                    descriptive_url("WKT2", "https://crs-explorer.proj.org/wkt2/EPSG/4978.txt"),
                ]),
                Wktf2Text::from(r##"
                    ELLIPSOID["WGS 84",6378137,298.257223563,
                        LENGTHUNIT["metre",1],
                        ID["EPSG",7030]
                    ]"##),
            ));
        }

            /*
                    {
                    "name": "World Geodetic System 1984 (G2139)",
                    "id": {
                        "authority": "EPSG",
                        "code": 1309
                    }
                }
            */


        //assert_ron_snapshot!(, @"");
        Ok(())
    }
}
