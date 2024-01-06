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
//? use std::collections::BTreeMap;
//? use std::fmt::{Debug, Display};
//? use std::ops::{RangeBounds, RangeInclusive};
//? use std::sync::{Arc, RwLock};
//? use std::time::Instant;

use anyhow::{anyhow, bail, ensure, Context, Result};
use derive_more::{Deref, DerefMut, Display, From, Into};
//? use enumflags2::{bitflags, make_bitflags, BitFlags};
use hecs::{Bundle, Entity, World};
use hecs_hierarchy::{Hierarchy, HierarchyMut, HierarchyQuery};
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

// Just a tag type for the namespace hierarchy.
// Not visible outside the crate. Maybe it will need to be?
pub(crate) struct Namespace;

/// An entity with a name, typically part of a [`Namespace`].
#[derive(Clone, Debug, Display, Deref, DerefMut, From, Into)]
pub struct Name(pub String);

impl std::convert::From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Name {
    pub fn entity_to_name_string(world: &World, entity: Entity) -> String {
        world
            .get::<&Name>(entity)
            .map(|name| name.to_string())
            .unwrap_or_else(|_| format!("¥{entity:?}"))
    }
}

/// A component of the [`hecs::Entity`] which is the root of the [`Namespace`].
/// It should not have a [`Name`] component, or anything else really.
/// Not a good idea to delete it.
#[derive(Clone, Copy, Debug, Display)]
pub struct NamespaceRoot;

/// Gets the [`NamespaceRoot`] [`hecs::Entity`].
pub fn ecs_ns_get_root(world: &World) -> Result<Entity> {
    let mut opt_root: Option<Entity> = None;
    for (e, _) in world.query::<&NamespaceRoot>().iter() {
        if opt_root.is_some() {
            bail!("Multiple namespace roots in World.");
        }

        opt_root = Some(e);
    }

    if let Some(e_root) = opt_root {
        Ok(e_root)
    } else {
        Err(anyhow!("Namespace root not found in World."))
    }
}

/// Gets the [`NamespaceRoot`] [`hecs::Entity`], or adds it if needed.
pub fn ecs_ns_get_or_create_root(world: &mut World) -> Result<Entity> {
    let mut opt_root: Option<Entity> = None;
    for (e, _) in world.query::<&NamespaceRoot>().iter() {
        if opt_root.is_some() {
            bail!("Multiple namespace roots in World.");
        }

        opt_root = Some(e);
    }

    Ok(opt_root.unwrap_or_else(|| world.spawn((NamespaceRoot,))))
}

pub(crate) fn ecs_add_stuff(world: &mut World) -> Result<()> {
    let e_root = ecs_ns_get_or_create_root(world)?;

    debug_assert_eq!(e_root, ecs_ns_get_root(world)?);

    Ok(())
}

#[derive(Clone, Copy)]
pub enum NamespaceIterItem {
    ParentEntry(Entity),
    LeafEntry(Entity),
    Leave,
}

pub fn ecs_ns_iter(world: &World) -> impl std::iter::IntoIterator<Item = NamespaceIterItem> {
    use crate::names::NamespaceIterItem::*;

    let mut v_out = vec![];

    if let Ok(e_root) = ecs_ns_get_root(world) {
        fn ecs_ns_iter_children(world: &World, e_parent: Entity, v_out: &mut Vec<NamespaceIterItem>) {
            for e_child in world.children::<Namespace>(e_parent) {
                let s_child = Name::entity_to_name_string(world, e_child);
                //eprintln!("Child: {e_child:?} {s_child}");

                if ecs_ns_has_some_children(world, e_child) {
                    v_out.push(ParentEntry(e_child));
                    ecs_ns_iter_children(world, e_child, v_out);
                    v_out.push(Leave);
                } else {
                    v_out.push(LeafEntry(e_child));
                }
            }
        }

        ecs_ns_iter_children(world, e_root, &mut v_out);
    }

    v_out
}

pub fn ecs_ns_has_some_children(world: &World, entity: Entity) -> bool {
    if let Ok(parent) = world.get::<&hecs_hierarchy::Parent<Namespace>>(entity) {
        //eprintln!("parent has {} children", parent.num_children());
        parent.num_children() != 0
    } else {
        //eprintln!("not a parent");
        false
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use cgmath::AbsDiffEq;
    use insta::assert_ron_snapshot;

    fn make_world() -> Result<World> {
        let mut world = World::default();
        //crate::ecs_add_stuff(&mut world);

        let r = ecs_ns_get_or_create_root(&mut world)?; //world.spawn(("r",));
        assert!(!ecs_ns_has_some_children(&world, r));

        let r_c1 = world.spawn((Name::from("r-c1"),));
        world.attach::<Namespace>(r_c1, r).unwrap();
        assert!(ecs_ns_has_some_children(&world, r));

        let r_c2 = world.spawn((Name::from("r-c2"),));
        world.attach::<Namespace>(r_c2, r).unwrap();

        let _child_1_1 = world
            .attach_new::<Namespace, _>(r_c1, (Name::from("r-c1-c1"),))
            .unwrap();

        //world.detach::<Namespace>(r_c1).unwrap();

        let _r_c3 = world
            .attach_new::<Namespace, _>(r, (Name::from("r-c3"),))
            .unwrap();

        //world.attach::<Namespace>(r_c1, _r_c3).unwrap();

        let _r_c4 = world
            .attach_new::<Namespace, _>(r, (Name::from("r-c4"),))
            .unwrap();

        //let r2 = world.spawn((Name::from("r2"),));
        let _r2_c1 = world
            .attach_new::<Namespace, _>(r, (Name::from("r2-c1"),))
            .unwrap();
        //let _r2_c1 = world.attach_new::<Namespace, _>(r2, (Name::from("r2-c1"),)).unwrap();

        Ok(world)
    }

    #[test]
    fn t1() -> anyhow::Result<()> {
        let world = make_world()?;

        eprintln!("Iterating roots and descendants recursively:");
        for (e_root, _) in world.roots::<Namespace>().unwrap().iter() {
            let s_root = Name::entity_to_name_string(&world, e_root);
            eprintln!("  Root: {e_root:?} {s_root}");

            for e_child in world.descendants_depth_first::<Namespace>(e_root) {
                let s_child = Name::entity_to_name_string(&world, e_child);
                let e_parent = world.parent::<Namespace>(e_child)?;
                let s_parent_name = Name::entity_to_name_string(&world, e_parent);
                eprintln!(
                    "    Descendant: {e_child:?} {s_child} (child of {e_parent:?} {s_parent_name})"
                );
            }
        }

        //? assert_ron_snapshot!(, @"");
        Ok(())
    }

    #[test]
    fn t2() -> anyhow::Result<()> {
        use crate::names::NamespaceIterItem::*;
        use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

        let world = make_world()?;

        eprintln!("Iterating roots and descendants recursively:");
        let depth = AtomicUsize::new(0);

        let mut indent = || {
            for _ in 0..depth.load(Relaxed) {
                eprint!(". ");
            }
        };

        for row in ecs_ns_iter(&world) {
            match row {
                ParentEntry(e) => {
                    indent();
                    eprintln!("P {}", Name::entity_to_name_string(&world, e));
                    depth.fetch_add(1, Relaxed);
                }
                LeafEntry(e) => {
                    indent();
                    eprintln!("L {}", Name::entity_to_name_string(&world, e));
                }
                Leave => {
                    indent();
                    eprintln!("V");
                    assert_ne!(depth.load(Relaxed), 0);
                    depth.fetch_sub(1, Relaxed);
                }
            }
        }
        assert_eq!(depth.load(Relaxed), 0);

        //? assert_ron_snapshot!(, @"");
        Ok(())
    }
}
