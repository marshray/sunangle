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

use anyhow::{anyhow, bail, ensure, Context, Result};
use egui::collapsing_header;
use hecs::{Bundle, Entity, World};
use hecs_hierarchy::{Hierarchy, HierarchyMut, HierarchyQuery};
//? use derive_more::{Deref, DerefMut, Display, From, Into};
use log::{debug, error, info, trace, warn};
//? use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::{NumCast, ToPrimitive, Zero};
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
//? use strum::{self, EnumCount, EnumDiscriminants, EnumProperty, EnumString, FromRepr};

use coordinate_systems::{names::NamespaceIterItem, *};

use crate::{ui::showable::ShowableEguiWindow, world_state};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EcsExploreWindow {}

impl EcsExploreWindow {
    const NAME_STR: &'static str = "ECS World";

    pub fn new() -> Self {
        Self {}
    }
}

impl ShowableEguiWindow for EcsExploreWindow {
    fn name(&self) -> Cow<'_, str> {
        Self::NAME_STR.into()
    }

    fn add_contents(&mut self, ui: &mut egui::Ui, world: &mut World) {
        let mut tr_iter = ecs_ns_iter(world).into_iter();

        //eprintln!("================== ShowableEguiWindow::add_contents");

        let mut depth = 0_usize;
        self.add_contents2(ui, world, &mut tr_iter, &mut depth);

        //eprintln!();
    }
}

impl EcsExploreWindow {
    #[allow(clippy::only_used_in_recursion)]
    fn add_contents2<I>(
        &mut self,
        ui: &mut egui::Ui,
        world: &mut World,
        tr_iter: &mut I,
        depth: &mut usize,
    ) where
        I: Iterator<Item = coordinate_systems::names::NamespaceIterItem>,
    {
        use coordinate_systems::names::NamespaceIterItem::*;

        let mut ignore_depth = 0_usize;

        while let Some(tr) = tr_iter.next() {
            //eprint!("\n{}add_contents2 d={depth}, id={ignore_depth}", ".   ".repeat(*depth));

            let opt_entity = match tr {
                ParentEntry(e) => Some(e),
                LeafEntry(e) => Some(e),
                Leave => None,
            };

            let opt_name = opt_entity.map(|e| Name::entity_to_name_string(world, e));

            let name = opt_name.clone().unwrap_or_default();

            let spc_name_s = opt_name
                .map(|s| format!(" {s:?}"))
                .unwrap_or_else(String::default);

            /*
            let prev_d = *depth;
            let prev_id = ignore_depth;
            match tr {
                ParentEntry(_) => {
                    eprint!(" ParentEntry{spc_name_s}")
                }
                LeafEntry(_) => {
                    eprint!(" LeafEntry{spc_name_s}")
                }
                Leave => {
                    eprint!(" Leave{spc_name_s}")
                }
            }
            // */

            match tr {
                ParentEntry(_) => {
                    *depth += 1;
                }
                LeafEntry(_) => {}
                Leave => {
                    *depth = depth.saturating_sub(1);
                }
            }

            /*
            if prev_d != *depth {
                eprint!(" d:{prev_d}->{depth}");
            }
            // */

            if 0 == ignore_depth {
                match tr {
                    ParentEntry(_) => {
                        let collapsing_response = ui.collapsing(name, |sub_ui| {
                            // eprint!(" (open)");
                            self.add_contents2(sub_ui, world, tr_iter, depth);
                        });

                        if collapsing_response.fully_closed() {
                            // eprint!(" fully_closed");
                            ignore_depth += 1;
                        }
                    }
                    LeafEntry(_) => {
                        ui.label(name);
                    }
                    Leave => {
                        //------------------------------ inner return
                        return;
                    }
                }
            } else {
                match tr {
                    ParentEntry(_) => {
                        ignore_depth += 1;
                    }
                    LeafEntry(_) => {}
                    Leave => {
                        ignore_depth -= 1;
                    }
                }
            }

            /*
            if prev_id != ignore_depth {
                eprint!(" id:{prev_id}->{ignore_depth}");
            }
            // */
        }
    }
}
