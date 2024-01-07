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
//? use derive_more::{Deref, DerefMut, Display};
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

pub type CowStaticStr = Cow<'static, str>;

pub type RatioU64 = num_rational::Ratio<u64>;
pub use num_rational::BigRational;

pub type Deg = cgmath::Deg<f64>;
pub type Rad = cgmath::Rad<f64>;

pub type Point1 = cgmath::Point1<f64>;
pub type Point2 = cgmath::Point2<f64>;
pub type Point3 = cgmath::Point3<f64>;

pub type Vector1 = cgmath::Vector1<f64>;
pub type Vector2 = cgmath::Vector2<f64>;
pub type Vector3 = cgmath::Vector3<f64>;
pub type Vector4 = cgmath::Vector4<f64>;

pub type Matrix2 = cgmath::Matrix2<f64>;
pub type Matrix3 = cgmath::Matrix3<f64>;
pub type Matrix4 = cgmath::Matrix4<f64>;

pub mod core;
pub use crate::core::{Abbr, DimensionKind, EcsNum, EcsNumRef, Exactness};

pub mod consts;
pub use crate::consts::DimensionedConstant;

pub mod names;
pub use crate::names::{
    ecs_add, ecs_ns_find_or_create, ecs_ns_has_some_children, ecs_ns_iter, Name, NamePathSpec,
    NamePathSpecStart, RootNamespace,
};

pub mod units;
pub use crate::units::{Unit, UnitDef};

pub mod geom;
pub use crate::geom::{
    ecs_add_oblatespheroid, OblateSpheroid, OblateSpheroidDef, OblateSpheroidRef,
};
//pub use crate::geom::{Ellipsoid3Sphere, Ellipsoid3Oblate, Ellipsoid3Triaxial};

pub mod gis;
pub use crate::gis::ecs_ns_find_or_create_gis;

pub mod gl;
pub use crate::gl::ecs_add_cs;

pub fn ecs_add_stuff(world: &mut hecs::World) {
    crate::names::ecs_add_stuff(world).unwrap(); // do first
    crate::consts::ecs_add_stuff(world).unwrap();
    crate::units::ecs_add_stuff(world).unwrap();
    crate::geom::ecs_add_stuff(world).unwrap();
    crate::gis::ecs_add_stuff(world).unwrap();
    crate::gl::ecs_add_stuff(world).unwrap();
}
