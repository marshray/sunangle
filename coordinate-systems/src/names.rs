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
use hecs::{Bundle, DynamicBundle, Entity, World};
use hecs_hierarchy::{Hierarchy, HierarchyMut, HierarchyQuery};
//? use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
//? use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

//-------------------------------------------------------------------------------------------------|

// Just a tag type for the namespace hierarchy.
// Not visible outside the crate. Maybe it will need to be?
pub(crate) struct Namespace;

//-------------------------------------------------------------------------------------------------|

/// An entity with a name, typically part of a [`Namespace`].
#[derive(Clone, Debug, Display, Deref, DerefMut, From, Into, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn opt_from_entity(world: &World, entity: Entity) -> Option<hecs::Ref<'_, Name>> {
        world.get::<&Name>(entity).ok()
        //.map(|n| &*n)
    }
}

//-------------------------------------------------------------------------------------------------|

/// A component of the [`hecs::Entity`] which is the root of the [`Namespace`].
/// It should not have a [`Name`] component, or anything else really.
/// Not a good idea to delete it.
#[derive(Clone, Copy, Debug, Display)]
pub struct RootNamespace;

impl RootNamespace {
    /// Gets the [`RootNamespace`] [`hecs::Entity`] from the [`hecs::World`],
    /// or returns [`None`].
    pub fn find_opt(world: &World) -> Option<Entity> {
        debug_assert!(
            world.query::<&RootNamespace>().iter().count() <= 1,
            "World has multiple `RootNamespace`s."
        );

        world
            .query::<&RootNamespace>()
            .iter()
            .next()
            .map(|(e, _)| e)
    }

    /// Gets the [`RootNamespace`] [`hecs::Entity`] from the [`hecs::World`].
    pub fn find(world: &World) -> Result<Entity> {
        Self::find_opt(world).ok_or_else(|| anyhow!("RootNamespace not found in World."))
    }

    /// Gets the [`NamespaceRoot`] [`hecs::Entity`] from the [`hecs::World`],
    /// or adds it.
    pub fn find_or_create(world: &mut World) -> Result<Entity> {
        debug_assert!(
            world.query::<&RootNamespace>().iter().count() <= 1,
            "World has multiple `RootNamespace`s."
        );

        let opt_pr = world.query_mut::<&RootNamespace>().into_iter().next();
        let e = if let Some(pr) = opt_pr {
            pr.0
        } else {
            world.spawn((RootNamespace,))
        };
        Ok(e)
    }
}

//-------------------------------------------------------------------------------------------------|

//-------------------------------------------------------------------------------------------------|

//=================================================================================================|

#[derive(Clone, Copy)]
pub enum NamePathSpecStart {
    Root,
    From(Entity),
}

impl NamePathSpecStart {
    pub fn to_entity_or_create(self, world: &mut World) -> Result<Entity> {
        use NamePathSpecStart::*;

        match self {
            Root => RootNamespace::find_or_create(world),
            From(e) => {
                debug_assert!(ecs_ns_entity_has_name(world, e)?);
                Ok(e)
            }
        }
    }

    pub fn try_to_entity(self, world: &World) -> Result<Entity> {
        use NamePathSpecStart::*;

        match self {
            Root => RootNamespace::find(world),
            From(e) => {
                debug_assert!(ecs_ns_entity_has_name(world, e)?);
                Ok(e)
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------|

pub struct NamePathSpec<N, II>
where
    N: Into<Name>,
    II: IntoIterator<Item = N>,
{
    start: NamePathSpecStart,
    components: II,
}

impl<N, II> NamePathSpec<N, II>
where
    N: Into<Name>,
    II: IntoIterator<Item = N>,
{
    pub fn absolute(components: II) -> Self {
        Self {
            start: NamePathSpecStart::Root,
            components,
        }
    }

    pub fn relative(e_from: Entity, components: II) -> Self {
        Self {
            start: NamePathSpecStart::From(e_from),
            components,
        }
    }
}

//-------------------------------------------------------------------------------------------------|

pub fn ecs_ns_find_or_create<N, II>(world: &mut World, nps: NamePathSpec<N, II>) -> Result<Entity>
where
    N: Into<Name>,
    II: IntoIterator<Item = N>,
{
    use NamePathSpecStart::*;

    let mut e = nps.start.to_entity_or_create(world)?;

    let mut it_components = nps.components.into_iter().fuse();

    #[allow(clippy::while_let_on_iterator)]
    'next_path_component: while let Some(path_component) = it_components.next() {
        let name: Name = path_component.into();

        for e_child in world.children::<Namespace>(e) {
            let opt_ref_child_name = Name::opt_from_entity(world, e_child);
            if let Some(ref_child_name) = opt_ref_child_name {
                if name == *ref_child_name {
                    e = e_child;
                    continue 'next_path_component;
                }
            }
        }

        break;
    }

    // Create any remaining name path components.
    for pc_into_name in it_components {
        let name: Name = pc_into_name.into();
        e = world.attach_new::<Namespace, _>(e, (name,))?;
    }

    Ok(e)
}

//-------------------------------------------------------------------------------------------------|

pub fn ecs_ns_get<N, II>(world: &mut World, nps: NamePathSpec<N, II>) -> Result<Entity>
where
    N: Into<Name>,
    II: IntoIterator<Item = N>,
{
    bail!("TODO"); //? TODO
}

//=================================================================================================|

pub fn ecs_ns_entity_has_name(world: &World, entity: Entity) -> Result<bool> {
    world
        .satisfies::<&Name>(entity)
        .map_err(|e| anyhow!("World satisfies entity {entity:?} has a Name component: {e}"))
}

//-------------------------------------------------------------------------------------------------|

pub fn ecs_ns_has_some_children(world: &World, entity: Entity) -> bool {
    if let Ok(parent) = world.get::<&hecs_hierarchy::Parent<Namespace>>(entity) {
        #[cfg(all(debug_print, debug_assertions))]
        eprintln!("parent has {} children", parent.num_children());
        parent.num_children() != 0
    } else {
        #[cfg(all(debug_print, debug_assertions))]
        eprintln!("not a parent");
        false
    }
}

//-------------------------------------------------------------------------------------------------|

#[derive(Clone, Copy)]
pub enum NamespaceIterItem {
    ParentEntry(Entity),
    LeafEntry(Entity),
    Leave,
}

pub fn ecs_ns_iter(world: &World) -> impl std::iter::IntoIterator<Item = NamespaceIterItem> {
    use crate::names::NamespaceIterItem::*;

    let mut v_out = vec![];

    if let Ok(e_root) = RootNamespace::find(world) {
        fn ecs_ns_iter_children(
            world: &World,
            e_parent: Entity,
            v_out: &mut Vec<NamespaceIterItem>,
        ) {
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

//=================================================================================================|

pub fn ecs_add<C: DynamicBundle>(
    world: &mut World,
    e_ns_parent: Entity,
    name: &str,
    components: C,
) -> Result<Entity> {
    world
        .attach_new::<Namespace, _>(e_ns_parent, components)
        .map_err(|e| anyhow!("World adding {name:?}: {e}"))
}

//=================================================================================================|
pub(crate) fn ecs_add_stuff(world: &mut World) -> Result<()> {
    RootNamespace::find_or_create(world)?;

    Ok(())
}

//=================================================================================================|

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;

    fn make_world() -> Result<World> {
        let mut world = World::default();

        let r = RootNamespace::find_or_create(&mut world)?; //world.spawn(("r",));
        assert!(!ecs_ns_has_some_children(&world, r));

        crate::ecs_add_stuff(&mut world);
        assert!(ecs_ns_has_some_children(&world, r));

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

        #[cfg(all(debug_print, debug_assertions))]
        eprintln!("Iterating roots and descendants recursively:");
        for (e_root, _) in world.roots::<Namespace>().unwrap().iter() {
            let s_root = Name::entity_to_name_string(&world, e_root);
            #[cfg(all(debug_print, debug_assertions))]
            eprintln!("  Root: {e_root:?} {s_root}");

            for e_child in world.descendants_depth_first::<Namespace>(e_root) {
                let s_child = Name::entity_to_name_string(&world, e_child);
                let e_parent = world.parent::<Namespace>(e_child)?;
                let s_parent_name = Name::entity_to_name_string(&world, e_parent);
                #[cfg(all(debug_print, debug_assertions))]
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

        #[cfg(all(debug_print, debug_assertions))]
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
                    #[cfg(all(debug_print, debug_assertions))]
                    eprintln!("P {}", Name::entity_to_name_string(&world, e));
                    depth.fetch_add(1, Relaxed);
                }
                LeafEntry(e) => {
                    indent();
                    #[cfg(all(debug_print, debug_assertions))]
                    eprintln!("L {}", Name::entity_to_name_string(&world, e));
                }
                Leave => {
                    indent();
                    #[cfg(all(debug_print, debug_assertions))]
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

    #[test]
    fn t3() -> anyhow::Result<()> {
        use crate::names::NamespaceIterItem::*;

        let world = &mut World::default();

        let ns_root = RootNamespace::find_or_create(world)?;

        //? assert_ron_snapshot!(, @"");
        Ok(())
    }
}
