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
use cgmath::Zero;
use derive_more::{Deref, DerefMut, Display, From, Into};
use hecs::{Bundle, Entity, World};
use hecs_hierarchy::{Hierarchy, HierarchyMut, HierarchyQuery};
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use ecs_namespace::NamespaceTag;
use crate::*;

//=================================================================================================|

#[derive(Clone, Debug, Display, Deref, DerefMut, From, Into)]
pub struct Abbr(pub String);

impl std::convert::From<&str> for Abbr {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

//=================================================================================================|

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum ExactReason {
    MeasuredWithBothInfinitePrecisionAndAccuracy,
    ByProof,
    ByDefinition,
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum Exactness {
    Exact(ExactReason),
    Approximate,
}

//=================================================================================================|

/// A numeric value or numeric-valued expression [`hecs::Component`].
///
/// This type does not support the usual `Eq` or `Ord` because that could require cloning of large
/// structures.
#[derive(Clone, Debug, Display)]
pub enum EcsNum {
    RatioU64(RatioU64),

    BigRational(BigRational),

    F64(f64),

    #[display("Entity({:?})", "_0")]
    Entity(Entity),

    #[display("Inverse({:?})", "_0")]
    Inverse(EcsNumRef),

    #[display("{:?}", "_0")]
    Ref(EcsNumRef),
}

impl EcsNum {
    pub fn recip(&self) -> Result<EcsNum> {
        use EcsNum::*;
        Ok(match self {
            RatioU64(r) if *r.denom() != 0 => RatioU64(r.recip()),
            BigRational(r) if !r.denom().is_zero() => BigRational(r.recip()),
            F64(f) if f.is_normal() => F64(f.recip()),
            Entity(e) => Inverse(EcsNumRef(*e)),
            Inverse(ecsnumref) => Ref(*ecsnumref),
            _ => {
                bail!("Can't compute reciprocal of {self:?}");
            }
        })
    }
}

//-------------------------------------------------------------------------------------------------|

/// Reference to a [`EcsNum`] [`hecs::Entity`] in the [`hecs::World`].
#[derive(Clone, Copy, Debug)]
pub struct EcsNumRef(Entity);

impl EcsNumRef {
    pub fn new(e: Entity, world: &World) -> Self {
        debug_assert!(
            world.satisfies::<&EcsNum>(e).unwrap_or_default(),
            "Although this newtype can't prevent the EcsNumRef Entity from being removed from the World, it should probably at least start out that way."
        );
        Self(e)
    }
}

//=================================================================================================|

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum DimensionKind {
    Length,
    Angle,
    Scale,
    Time,
}

//=================================================================================================|
