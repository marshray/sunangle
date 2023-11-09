// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ops::RangeInclusive;

use num_traits::cast::NumCast;
use serde::{Deserialize, Serialize};

use crate::time::day_ops::DayOps;
use crate::time::{Error, Result};

/// A possibly valid day number, 1 through 31.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Day(pub u8);

impl Day {
    /// The min valid 1-based day number.
    ///
    /// `1`
    pub const MIN: u8 = 1;

    /// The max valid 1-based day number.
    ///
    /// `31`
    pub const MAX: u8 = 31;

    /// The valid 1-based day number range.
    ///
    /// `1 ..= 31`
    pub const RI: RangeInclusive<u8> = Self::MIN..=Self::MAX;

    /// Returns a [`Day`] iff `d` is a valid 1-based day number.
    pub fn try_new<T: NumCast + Copy>(d: T) -> Result<Self> {
        let Some(d_u8) = NumCast::from(d) else {
            return Err(Error::InvalidDay(NumCast::from(d).unwrap_or(isize::MIN)));
        };

        if !Self::RI.contains(&d_u8) {
            return Err(Error::InvalidDay(d_u8.into()));
        }

        Ok(Day(d_u8))
    }
}

impl DayOps for Day {
    fn day(&self) -> Day {
        *self
    }
}

impl From<Day> for u8 {
    fn from(d: Day) -> u8 {
        d.0
    }
}

impl From<Day> for i8 {
    fn from(d: Day) -> i8 {
        d.0 as i8
    }
}
        
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        insta::assert_ron_snapshot!(Day::try_new(0), @"Err(InvalidDay(0))");
        insta::assert_ron_snapshot!(Day::try_new(1), @"Ok(Day(1))");
        insta::assert_ron_snapshot!(Day::try_new(31), @"Ok(Day(31))");
        insta::assert_ron_snapshot!(Day::try_new(32), @"Err(InvalidDay(32))");
    }

    #[test]
    fn test_day_ops() {
        let day = Day(15);
        assert_eq!(day.day_as_one_based_u8(), 15);
        assert_eq!(day.day_as_one_based_u8(), day.into());
        assert_eq!(day.day_as_one_based_u8() as i8, day.into());
        let day_from_trait = day.day();
        assert_eq!(day_from_trait.0, 15);
    }
}
