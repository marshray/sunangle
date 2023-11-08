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

//? use use std::fmt::Display;
use std::ops::RangeInclusive;

use num_traits::cast::NumCast;
use serde::{Deserialize, Serialize};

use crate::time::gregorian::GregorianYear;
use crate::time::year_ops::YearOps;
use crate::time::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A valid and supported Astro year.
///
/// The [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering)
/// system is just the conventional numbering system with values less than 1 shifted up by 1 to
/// eliminate the gap at zero.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct AstroYear(pub i32);

impl AstroYear {
    /// The min astro year supported.
    ///
    /// `GregorianYear::MIN + 1 = -8190`
    pub const MIN: i32 = -8191;

    /// The max astro year supported.
    ///
    /// `GregorianYear::MAX = 8191`
    pub const MAX: i32 = 8191;

    /// The supported astro year range.
    ///
    /// `-8191 ..= 8191`
    ///
    /// Note that `0` IS valid.
    pub const RI: RangeInclusive<i32> = Self::MIN..=Self::MAX;

    /// Returns an `AstroYear` iff `y` is a supported astro year.
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

        Ok(AstroYear(y_i32))
    }
}

impl YearOps for AstroYear {
    fn gregorian_year(&self) -> GregorianYear {
        (*self).into()
    }

    fn astro_year(&self) -> AstroYear {
        *self
    }

    fn gregorian_year_i32(&self) -> i32 {
        self.gregorian_year().gregorian_year_i32()
    }

    fn astro_year_i32(&self) -> i32 {
        self.0
    }

    fn is_leap_year(&self) -> bool {
        let ay = self.0;
        0 == ay % 4 && (0 != ay % 100 || 0 == ay % 400)
    }
}

/* impl TryFrom<i32> for AstroYear {
    type Error = Error;

    fn try_from(y: i32) -> Result<Self> {
        if !Self::RI.contains(&y) {
            Err(Error::UnsupportedYear(y))
        } else {
            Ok(AstroYear(y))
        }
    }
} */

impl From<GregorianYear> for AstroYear {
    fn from(gy: GregorianYear) -> Self {
        let y = gy.0;
        let y = if 0 < y {
            // Positive year
            y
        } else {
            debug_assert!(y != 0, "violated invariant of GregorianYear");
            y + 1
        };
        debug_assert!(
            GregorianYear::RI.contains(&y),
            "violated invariant of GregorianYear"
        );
        AstroYear(y)
    }
}

#[cfg(test)]
mod tay {
    use super::*;

    #[test]
    fn t0() {
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MIN - 1), @"Err(UnsupportedYear(-8192))");
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MIN), @"Ok(AstroYear(-8191))");
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MAX), @"Ok(AstroYear(8191))");
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MAX + 1), @"Err(UnsupportedYear(8192))");
    }

    #[test]
    fn t1() {
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MIN - 1), @"Err(UnsupportedYear(-8192))");
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MIN), @"Ok(AstroYear(-8191))");
        insta::assert_ron_snapshot!(AstroYear::try_new(-1), @"Ok(AstroYear(-1))");
        insta::assert_ron_snapshot!(AstroYear::try_new(0), @"Ok(AstroYear(0))");
        insta::assert_ron_snapshot!(AstroYear::try_new(1), @"Ok(AstroYear(1))");
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MAX), @"Ok(AstroYear(8191))");
        insta::assert_ron_snapshot!(AstroYear::try_new(AstroYear::MAX + 1), @"Err(UnsupportedYear(8192))");
    }

    #[test]
    fn t2() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(AstroYear::from(GregorianYear::try_new(GregorianYear::MIN)?), @"AstroYear(-8191)");
        insta::assert_ron_snapshot!(AstroYear::from(GregorianYear::try_new(-1)?), @"AstroYear(0)");
        insta::assert_ron_snapshot!(AstroYear::from(GregorianYear::try_new(1)?), @"AstroYear(1)");
        insta::assert_ron_snapshot!(AstroYear::from(GregorianYear::try_new(GregorianYear::MAX)?), @"AstroYear(8191)");
        insta::assert_ron_snapshot!(AstroYear::from(GregorianYear::try_new(GregorianYear::MAX)?), @"AstroYear(8191)");
        Ok(())
    }

    #[test]
    fn t3() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(AstroYear::try_new(-2000)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(-104)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(-100)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(-96)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(-5)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(-4)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(-3)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(-2)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(-1)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(0)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(1)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(2)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(3)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(4)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(5)?.is_leap_year(), @"false");

        insta::assert_ron_snapshot!(AstroYear::try_new(1899)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(1900)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(1901)?.is_leap_year(), @"false");

        insta::assert_ron_snapshot!(AstroYear::try_new(1999)?.is_leap_year(), @"false");
        insta::assert_ron_snapshot!(AstroYear::try_new(2000)?.is_leap_year(), @"true");
        insta::assert_ron_snapshot!(AstroYear::try_new(2001)?.is_leap_year(), @"false");

        Ok(())
    }

    #[test]
    fn t4() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(AstroYear::try_new(1999)?.days_in_year(), @"365");
        insta::assert_ron_snapshot!(AstroYear::try_new(2000)?.days_in_year(), @"366");
        insta::assert_ron_snapshot!(AstroYear::try_new(2001)?.days_in_year(), @"365");

        insta::assert_ron_snapshot!(AstroYear::try_new(2003)?.days_in_year(), @"365");
        insta::assert_ron_snapshot!(AstroYear::try_new(2004)?.days_in_year(), @"366");
        insta::assert_ron_snapshot!(AstroYear::try_new(2005)?.days_in_year(), @"365");

        Ok(())
    }
}
