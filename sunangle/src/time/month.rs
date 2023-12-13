// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ops::RangeInclusive;

use num_traits::cast::NumCast;
use serde::{Deserialize, Serialize};

use crate::time::month_ops::MonthOps;
use crate::time::{Error, Result};

/// A valid month.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Month(pub u8);

impl Month {
    /// The min valid 1-based month number.
    ///
    /// `1`
    pub const MIN: u8 = 1;

    /// The max valid 1-based month number.
    ///
    /// `12`
    pub const MAX: u8 = 12;

    /// The valid 1-based month number range.
    ///
    /// `1 ..= 12`
    ///
    pub const RI: RangeInclusive<u8> = Self::MIN..=Self::MAX;

    /// Returns a `Month` iff `m` is a valid 1-based month number.
    pub fn try_new<T: NumCast + Copy>(m: T) -> Result<Self> {
        let Some(m_u8) = NumCast::from(m) else {
            return Err(Error::InvalidMonth(NumCast::from(m).unwrap_or(isize::MIN)));
        };

        if !Self::RI.contains(&m_u8) {
            return Err(Error::InvalidMonth(m_u8.into()));
        }

        Ok(Month(m_u8))
    }
}

impl MonthOps for Month {
    fn month(&self) -> Month {
        *self
    }
}

impl From<Month> for u8 {
    fn from(mo: Month) -> u8 {
        mo.0
    }
}

impl From<Month> for i8 {
    fn from(mo: Month) -> i8 {
        mo.0 as i8
    }
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::time::astro_year::AstroYear;

    #[test]
    fn t0() {
        insta::assert_ron_snapshot!(Month::try_new(0), @"Err(InvalidMonth(0))");
        insta::assert_ron_snapshot!(Month::try_new(1), @"Ok(Month(1))");
        insta::assert_ron_snapshot!(Month::try_new(12), @"Ok(Month(12))");
        insta::assert_ron_snapshot!(Month::try_new(13), @"Err(InvalidMonth(13))");
    }

    #[test]
    fn t1() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(Month::try_new(1)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        insta::assert_ron_snapshot!(Month::try_new(2)?.days_in_month(&AstroYear::try_new(2020)?), @"29");
        insta::assert_ron_snapshot!(Month::try_new(2)?.days_in_month(&AstroYear::try_new(2021)?), @"28");
        insta::assert_ron_snapshot!(Month::try_new(3)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        insta::assert_ron_snapshot!(Month::try_new(4)?.days_in_month(&AstroYear::try_new(2020)?), @"30");
        insta::assert_ron_snapshot!(Month::try_new(5)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        insta::assert_ron_snapshot!(Month::try_new(6)?.days_in_month(&AstroYear::try_new(2020)?), @"30");
        insta::assert_ron_snapshot!(Month::try_new(7)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        insta::assert_ron_snapshot!(Month::try_new(8)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        insta::assert_ron_snapshot!(Month::try_new(9)?.days_in_month(&AstroYear::try_new(2020)?), @"30");
        insta::assert_ron_snapshot!(Month::try_new(10)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        insta::assert_ron_snapshot!(Month::try_new(11)?.days_in_month(&AstroYear::try_new(2020)?), @"30");
        insta::assert_ron_snapshot!(Month::try_new(12)?.days_in_month(&AstroYear::try_new(2020)?), @"31");
        Ok(())
    }

    #[test]
    fn month_ops() {
        let mo = Month(7);
        assert_eq!(mo.month_as_one_based_u8(), 7);
        assert_eq!(mo.month_as_one_based_u8(), mo.into());
        assert_eq!(mo.month_as_one_based_u8() as i8, mo.into());
        let month_from_trait = mo.month();
        assert_eq!(month_from_trait.0, 7);
    }
}
