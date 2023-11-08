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

use std::ops::RangeInclusive;

use num_traits::cast::NumCast;
use serde::{Deserialize, Serialize};

use crate::time::astro_year::AstroYear;
use crate::time::year_ops::YearOps;
use crate::time::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A valid and supported Gregorian (proleptic) year.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GregorianYear(pub i32);

impl GregorianYear {
    /// The min supported Gregorian year.
    ///
    /// `AstroYear::MIN - 1 = -8192`
    pub const MIN: i32 = AstroYear::MIN - 1;

    /// The max supported Gregorian year.
    ///
    /// `AstroYear::MAX = 8191`
    pub const MAX: i32 = AstroYear::MAX;

    /// The supported Gregorian year range.
    ///
    /// `-8191 ..= 8191`
    ///
    /// Note that `0` is invalid.
    pub const RI: RangeInclusive<i32> = Self::MIN..=Self::MAX;

    /// Returns an `Gregorian` iff `year`` is a valid and supported Gregorian (proleptic) year.
    ///
    /// `0` is the only invalid year.
    ///
    /// Unsupported years are those outside `[MIN, MAX]`.
    pub fn try_new<T: NumCast + Copy>(y: T) -> Result<Self> {
        let Some(y_i32) = NumCast::from(y) else {
            return Err(Error::UnsupportedYear(NumCast::from(y).unwrap_or(isize::MIN)));
        };

        if !Self::RI.contains(&y_i32) {
            return Err(Error::UnsupportedYear(
                NumCast::from(y).unwrap_or(isize::MIN),
            ));
        }

        if y_i32 == 0 {
            return Err(Error::InvalidGregorianYear(
                NumCast::from(y).unwrap_or(isize::MIN),
            ));
        }

        Ok(GregorianYear(y_i32))
    }
}

impl YearOps for GregorianYear {
    fn gregorian_year(&self) -> GregorianYear {
        *self
    }

    fn astro_year(&self) -> AstroYear {
        (*self).into()
    }

    fn gregorian_year_i32(&self) -> i32 {
        self.0
    }

    fn astro_year_i32(&self) -> i32 {
        self.gregorian_year().gregorian_year_i32()
    }

    fn is_leap_year(&self) -> bool {
        AstroYear::from(*self).is_leap_year()
    }
}

impl From<AstroYear> for GregorianYear {
    fn from(ay: AstroYear) -> Self {
        let y = ay.0;
        let y = if 0 < y {
            // Positive year
            y
        } else {
            // Non-positive year
            y - 1
        };
        debug_assert!(Self::RI.contains(&y), "violated invariant of AstroYear");
        GregorianYear(y)
    }
}

#[cfg(test)]
mod tgy {
    use super::*;

    #[test]
    fn t0() {
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MIN - 1), @"Err(UnsupportedYear(-8193))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MIN), @"Ok(GregorianYear(-8192))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MAX), @"Ok(GregorianYear(8191))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MAX + 1), @"Err(UnsupportedYear(8192))");
    }

    #[test]
    fn t1() {
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MIN - 1), @"Err(UnsupportedYear(-8193))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MIN), @"Ok(GregorianYear(-8192))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(-1), @"Ok(GregorianYear(-1))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(0), @"Err(InvalidGregorianYear(0))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(1), @"Ok(GregorianYear(1))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MAX), @"Ok(GregorianYear(8191))");
        insta::assert_ron_snapshot!(GregorianYear::try_new(GregorianYear::MAX + 1), @"Err(UnsupportedYear(8192))");
    }

    #[test]
    fn t2() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(GregorianYear::from(AstroYear::try_new(AstroYear::MIN)?), @"GregorianYear(-8192)");
        insta::assert_ron_snapshot!(GregorianYear::from(AstroYear::try_new(-1)?), @"GregorianYear(-2)");
        insta::assert_ron_snapshot!(GregorianYear::from(AstroYear::try_new(0)?), @"GregorianYear(-1)");
        insta::assert_ron_snapshot!(GregorianYear::from(AstroYear::try_new(1)?), @"GregorianYear(1)");
        insta::assert_ron_snapshot!(GregorianYear::from(AstroYear::try_new(AstroYear::MAX)?), @"GregorianYear(8191)");
        Ok(())
    }

    #[test]
    fn t3() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(GregorianYear::try_new(-6)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(-5)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(GregorianYear::try_new(-4)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(-3)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(-2)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(-1)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(GregorianYear::try_new(1)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(2)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(3)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(4)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(GregorianYear::try_new(5)?.is_leap_year(), @"false");

        insta::assert_ron_snapshot!(GregorianYear::try_new(1899)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(1900)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(1901)?.is_leap_year(), @"false");

        insta::assert_ron_snapshot!(GregorianYear::try_new(1999)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(GregorianYear::try_new(2000)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(GregorianYear::try_new(2001)?.is_leap_year(), @"false");

        Ok(())
    }

    #[test]
    fn t4() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(GregorianYear::try_new(1999)?.days_in_year(), @"365");
        insta::assert_ron_snapshot!(GregorianYear::try_new(2000)?.days_in_year(), @"366");
        insta::assert_ron_snapshot!(GregorianYear::try_new(2001)?.days_in_year(), @"365");

        Ok(())
    }
}
