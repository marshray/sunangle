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
//? use derive_more::Display;
//? use log::{debug, error, info, trace, warn};
use num_enum::{IntoPrimitive, TryFromPrimitive};
//? use num_integer::Integer;
//? use num_rational::Ratio;
//? use num_traits::identities::Zero;
use num_traits::{NumCast, ToPrimitive};
//? use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum::{self, EnumCount, EnumIter, EnumDiscriminants, EnumProperty, EnumString, FromRepr, IntoEnumIterator};

use video::Timecode;

use crate::tai::DateTimeTai;

/// The state of a [`DrawFrameInfo`].
#[derive(Debug, EnumIter, IntoPrimitive, EnumCount, TryFromPrimitive)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum DrawFrameState {
    Initial,
    UiUpdateStarted,
    UiUpdateFinished,
    PaintStarted,
    PaintFinished,
}

/// Infomation about the state of a frame to be drawn.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct DrawFrameInfo {
    v_state_entry_times: Vec<DateTimeTai>,
    //state_info: DrawFrameState,
    
    opt_frame_nr: Option<u64>,
    opt_timecode: Option<Timecode>,
}

impl DrawFrameInfo {
    /// Creates a new [`DrawFrameInfo`] having entering `Initial` state now.
    pub fn new() -> Self {
        Self::new_at(DateTimeTai::now())
    }

    /// Creates a new [`DrawFrameInfo`] having entering `Initial` state at the specified time.
    pub fn new_at(tai: DateTimeTai) -> Self {
        Self {
            v_state_entry_times: vec![tai],
            opt_frame_nr: None,
            opt_timecode: None,
        }
    }

    /// Checks if the frame is in the specified state.
    pub fn is_state(&self, s: DrawFrameState) -> bool {
        let s_u8: u8 = s.into();
        let v_len: usize = self.v_state_entry_times.len();
        let v_st_u8 = v_len.saturating_sub(1).to_u8().unwrap();
        s_u8 == v_st_u8
    }

    /// Gets the frame's current state.
    pub fn state(&self) -> DrawFrameState {
        let v_len = self.v_state_entry_times.len();
        let v_st_u8 = v_len.saturating_sub(1).to_u8().unwrap();
        DrawFrameState::try_from_primitive(v_st_u8).unwrap()
    }

    /// Gets the entry time to the current state.
    pub fn state_entry_time_tai(&self, st: DrawFrameState) -> Option<DateTimeTai> {
        let st_usize: usize = Into::<u8>::into(st).into();
        self.v_state_entry_times.get(st_usize).cloned()
    }

    /// Gets the state entry times, or `None` for states that have not yet been entered.
    pub fn state_entry_times_tai(&self, st: DrawFrameState) -> Vec::<(DrawFrameState, Option<DateTimeTai>)> {
        DrawFrameState::iter().enumerate()
            .map(|(ix, st)| (st, self.v_state_entry_times.get(ix).cloned()))
            .collect()
    }

    /// Gets the frame's [`video::Timecode`].
    pub fn timecode(&self) -> Result<Timecode> {
        self.opt_timecode.ok_or_else(|| anyhow!("No timecode"))
    }

    /// Gets the frame's optional [`video::Timecode`].
    pub fn opt_timecode(&self) -> Option<Timecode> {
        self.opt_timecode
    }

    /// Sets the frame's optional [`video::Timecode`].
    pub fn set_opt_timecode(&mut self, opt_timecode: Option<Timecode>) {
        self.opt_timecode = opt_timecode;
    }

    /// Sets the frame's [`video::Timecode`].
    pub fn set_timecode(&mut self, timecode: Timecode) {
        self.opt_timecode = Some(timecode);
    }

    /// Returns the optional [`egui::frame_nr`].
    pub fn opt_frame_nr(&self) -> Option<u64> { self.opt_frame_nr }

    /// Returns the [`egui::frame_nr`], or `Err`.
    pub fn frame_nr(&self) -> Result<u64> {
        self.opt_frame_nr.ok_or_else(|| anyhow!("No frame_nr"))
    }

    /// Indicates that the ui update function is being started.
    pub fn start_ui_update(&mut self, frame_nr: u64) -> Result<()> {
        let tai = if !self.is_state(DrawFrameState::Initial) {
            *self = DrawFrameInfo::new();
            self.v_state_entry_times[0]
        } else {
            DateTimeTai::now()
        };

        self.opt_frame_nr = Some(frame_nr);
        self.v_state_entry_times.push(tai);
        debug_assert!(self.is_state(DrawFrameState::UiUpdateStarted));
        
        Ok(())
    }

    /// Indicates that the ui update function is being finished.
    pub fn finish_ui_update(&mut self) -> Result<()> {
        ensure!(self.is_state(DrawFrameState::UiUpdateStarted));
        self.v_state_entry_times.push(DateTimeTai::now());
        debug_assert!(self.is_state(DrawFrameState::UiUpdateFinished));
        Ok(())
    }

    /// Indicates that the paint function is being started.
    pub fn start_paint(&mut self) -> Result<()> {
        ensure!(self.is_state(DrawFrameState::UiUpdateFinished));
        self.v_state_entry_times.push(DateTimeTai::now());
        debug_assert!(self.is_state(DrawFrameState::PaintStarted));
        Ok(())
    }

    /// Indicates that the paint function is being finished.
    pub fn finish_paint(&mut self) -> Result<()> {
        ensure!(self.is_state(DrawFrameState::PaintStarted));
        self.v_state_entry_times.push(DateTimeTai::now());
        debug_assert!(self.is_state(DrawFrameState::PaintFinished));
        Ok(())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod t {
    use super::*;
    use insta::assert_ron_snapshot;
    use video::TimecodeKind;

    #[test]
    fn t() -> anyhow::Result<()> {
        let mut dfi = DrawFrameInfo::new_at(DateTimeTai::EPOCH_400);
        assert_ron_snapshot!(dfi.state(), @"Initial");

        dfi.set_timecode(Timecode::new(&video::TIMECODEKIND_60_FPS));

        dfi.start_ui_update(123)?;
        assert_ron_snapshot!(dfi.state(), @"UiUpdateStarted");

        dfi.finish_ui_update()?;
        assert_ron_snapshot!(dfi.state(), @"UiUpdateFinished");

        dfi.start_paint()?;
        assert_ron_snapshot!(dfi.state(), @"PaintStarted");

        dfi.finish_paint()?;
        assert_ron_snapshot!(dfi.state(), @"PaintFinished");

        //? assert_ron_snapshot!(, @"");
        Ok(())
    }
}