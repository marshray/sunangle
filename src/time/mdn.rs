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

use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::time::astro_year::AstroYear;
use crate::time::day::Day;
use crate::time::day_ops::DayOps;
use crate::time::gregorian::{self, GregorianYear};
use crate::time::month::Month;
use crate::time::month_ops::MonthOps;
use crate::time::year_ops::YearOps;
use crate::time::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Millennium day number
///
/// An integer corresponding to the sequence:
/// ```ignore
/// 2000-02-29 = -1,
/// 2000-03-01 =  0,
/// 2000-03-02 =  1.
/// ```
///
/// Sometimes this is referred to as a "Julian date" system, but this terminology appears
/// mostly unrelated to the Julian calendar.
///
/// Design rationale: March 1, 2000 is the beginning of the nearest 400-year leap day cycle,
/// which (slightly) simplifies calculations.
///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Mdn(pub i32);

impl Mdn {
    /// The first supported `Mdn` day number.
    ///
    /// `-3722246` = Jan 1, 8192 BCE
    pub const MIN: i32 = -3722246; // Jan 1, 8192 BCE

    /// The last supported `Mdn` day number.
    ///
    /// `-2261521` = Dec 31, AD 8191
    pub const MAX: i32 = 2261521;

    //? TODO: why not a greater range?
    /// The full supported `Mdn` day number range.
    pub const RI: RangeInclusive<i32> = Self::MIN..=Self::MAX;

    const CNT_DAYS_IN_MOST_1_YEARS: i32 = 365;
    const CNT_DAYS_IN_MOST_4_YEARS: i32 = 4 * Self::CNT_DAYS_IN_MOST_1_YEARS + 1;
    const CNT_DAYS_IN_MOST_100_YEARS: i32 = 25 * Self::CNT_DAYS_IN_MOST_4_YEARS - 1;
    const CNT_DAYS_IN_400_YEARS: i32 = 4 * Self::CNT_DAYS_IN_MOST_100_YEARS + 1;

    /// Creates the `Mdn` corresponding to the specified `GregorianYear`, `Month`, and `Day` typed values.
    pub fn try_from_ymd<GY: YearOps + Copy, M: MonthOps + Copy, D: DayOps + Copy>(
        gy: GY,
        m: M,
        d: D,
    ) -> Result<Mdn> {
        let mut gy = gy.gregorian_year_i32();
        let m: i32 = m.month_as_one_based_u8().into();
        let d: i32 = d.day_as_one_based_u8().into();
        Self::try_from_gymd_nums(gy, m, d)
    }

    /// Creates the `Mdn` corresponding to the specified Gregorian year, month, and day numeric values.
    pub fn try_from_gymd_nums<
        GY: ToPrimitive + Copy,
        M: ToPrimitive + Copy,
        D: ToPrimitive + Copy,
    >(
        gy: GY,
        m: M,
        d: D,
    ) -> Result<Mdn> {
        let mut gy = gy.to_i32().ok_or(Error::InvalidGregorianYear(
            gy.to_isize().unwrap_or(isize::MIN),
        ))?;
        let m = m
            .to_i32()
            .ok_or(Error::InvalidMonth(gy.to_isize().unwrap_or(isize::MIN)))?;
        let d = d
            .to_i32()
            .ok_or(Error::InvalidDay(gy.to_isize().unwrap_or(isize::MIN)))?;
        Self::try_from_gymd_i32s(gy, m, d)
    }

    /// Creates the `Mdn` corresponding to the specified Gregorian year, month, and day `i32` values.
    pub fn try_from_gymd_i32s(gy: i32, m: i32, d: i32) -> Result<Mdn> {
        let mut gy = gy;

        // Adapted from the formulae:
        //		"367*Y - 7*(Y + (M + 9)/12)/4 - 3*((Y + (M - 9)/7)/100 + 1)/4 + 275*M/9 + D - 1721029"
        //		"367*Y - 7*[Y + (M + 9)/12]/4 + 275*M/9 + D + 1721014"
        // Van Flandern, T. C.; Pulkkinen, K. F., Astrophysical Journal Supplement Series, vol. 41, Nov. 1979, p. 391-411.
        // http://adsabs.harvard.edu/abs/1979ApJS...41..391V

        let mdn = if 1900 < gy && gy < 2100 {
            367 * gy - 7 * (gy + (m + 9) / 12) / 4 + 275 * m / 9 + d - 730591
        } else {
            //	Adjust year upward by multiples of 400 years.
            let mut n = 0;
            if gy < 0 {
                gy += 1; // no year 0
                n = (-gy + 399) / 400;
                gy += 400 * n;
            }

            367 * gy - 7 * (gy + (m + 9) / 12) / 4 - 3 * ((gy + (m - 9) / 7) / 100 + 1) / 4
                + 275 * m / 9
                + d
                - 730576
                - n * Self::CNT_DAYS_IN_400_YEARS
        };

        Ok(Mdn(mdn))
    }

    /// Creates an `Mdn` if the supplied value is within range.
    fn try_new<T: ToPrimitive + Copy>(i: T) -> Result<Self> {
        let i = i
            .to_i32()
            .ok_or(Error::OutOfMdnRange(i.to_isize().unwrap_or(isize::MIN)))?;
        if Self::RI.contains(&i) {
            Ok(Mdn(i))
        } else {
            Err(Error::OutOfMdnRange(i.to_isize().unwrap_or(isize::MIN)))
        }
    }

    /// Returns the `GregorianYear`, `Month`, and `Day` values.
    pub fn to_gymd(self) -> (GregorianYear, Month, Day) {
        let mut d1: i32 = self.0;

        // Find astro year of nearest March 1 not after target mdn and the days relative to it.
        //	Find the block of 400 years.
        let b400: i32 = if 0 <= d1 {
            d1 / Self::CNT_DAYS_IN_400_YEARS
        } else {
            // adjust earlier dates into range of the 400-year block beginning March 1, 2000.
            -(-d1 + Self::CNT_DAYS_IN_400_YEARS - 1) / Self::CNT_DAYS_IN_400_YEARS
        };

        d1 -= b400 * Self::CNT_DAYS_IN_400_YEARS;

        debug_assert!((0..Self::CNT_DAYS_IN_400_YEARS).contains(&d1));

        // Find the block of 100 years.
        let mut b100: i32 = d1 / Self::CNT_DAYS_IN_MOST_100_YEARS;
        b100 = b100.min(3); // Ensure the last day of the 400 year cycle stays in the 4th block.
        d1 -= b100 * Self::CNT_DAYS_IN_MOST_100_YEARS;

        debug_assert!((0..(Self::CNT_DAYS_IN_MOST_100_YEARS + 1)).contains(&d1));

        // Find the block of 4 years. The 100 year cycle is short a day, so we don't have to worry about overrun.
        let mut b4: i32 = d1 / Self::CNT_DAYS_IN_MOST_4_YEARS;
        d1 -= b4 * Self::CNT_DAYS_IN_MOST_4_YEARS;

        debug_assert!((0..Self::CNT_DAYS_IN_MOST_4_YEARS).contains(&d1));

        // Find the year within the 4 year cycle.
        let mut b1: i32 = d1 / Self::CNT_DAYS_IN_MOST_1_YEARS;
        b1 = b1.min(3); // Ensure the last day of the 4 year cycle stays in the 4th block.
        d1 -= b1 * Self::CNT_DAYS_IN_MOST_1_YEARS;

        // Now we know the astro year.
        let ay_i32: i32 = 2000 + 400 * b400 + 100 * b100 + 4 * b4 + b1;

        // #ifndef NDEBUG // Check ay.
        /* if (gregorian::MIN < ay && ay < gregorian::MAX) // avoid corner cases
        {
            // Target mdn should be between within [ March 1 of yonm1naftmdn, March 1 of yonm1naftmdn+1 ).
            let yonm1naftmdn: i32 = gregorian::astro_year_to_year(ay);
            let yonm1naftmdn_p1: i32 = gregorian::astro_year_to_year(ay + 1);
            let mdn_yonm1naftmdn_3_1: i32 = ymd_to_mdn(yonm1naftmdn, 3, 1);
            let mdn_yonm1naftmdnp1_3_1: i32 = ymd_to_mdn(yonm1naftmdn_p1, 3, 1);
            debug_assert!(mdn_yonm1naftmdn_3_1 <= mdn && mdn < mdn_yonm1naftmdnp1_3_1);
        } */
        // #endif // ndef NDEBUG

        //	At this point:
        //          d1   month  day
        //           0     Mar     1
        //         365     Feb    29    of following year

        let m1: i32 = (d1 + d1 / 61 - d1 / 183 + d1 / 214 - d1 / 244 + d1 / 275 - d1 / 305) / 31;
        debug_assert!(m1 < 12);

        // Jan and Feb belong to the next year.
        let ay = AstroYear::try_new(ay_i32 + ((m1 / 10) & 1)).unwrap();

        // Remove from d1 the day offset of the beginning of month m1.
        let m_offset_d: i32 = 61 * (m1 / 2) + 31 * (m1 & 1) + ((1 + m1) & 1) * (m1 / 6) + m1 / 11;
        debug_assert_eq!(
            m_offset_d,
            match m1 {
                0 => 0,
                1 => 31,
                2 => 61,
                3 => 92,
                4 => 122,
                5 => 153,
                6 => 184,
                7 => 214,
                8 => 245,
                9 => 275,
                10 => 306,
                _ => 337,
            }
        );

        let gy: GregorianYear = ay.into();

        // Translate m from (0=Mar, 11=Feb) to (1=Jan, 12=Dec).
        let m = Month::try_new(1 + (m1 + 2) % 12).unwrap();

        // Apply 1-based numbering to day.
        let d = Day::try_new(d1 - m_offset_d + 1).unwrap();

        (gy, m, d)
    }
}

impl From<i16> for Mdn {
    fn from(i: i16) -> Self {
        Mdn(i.into())
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn t0() {
        insta::assert_ron_snapshot!(Mdn::from(i16::MIN), @"Mdn(-32768)");
        insta::assert_ron_snapshot!(Mdn::from(-1_i16), @"Mdn(-1)");
        insta::assert_ron_snapshot!(Mdn::from(0_i16), @"Mdn(0)");
        insta::assert_ron_snapshot!(Mdn::from(1_i16), @"Mdn(1)");
        insta::assert_ron_snapshot!(Mdn::from(i16::MAX), @"Mdn(32767)");
    }

    #[test]
    fn t1() {
        insta::assert_ron_snapshot!(Mdn::try_new(Mdn::MIN - 1), @"Err(OutOfMdnRange(-3722247))");
        insta::assert_ron_snapshot!(Mdn::try_new(Mdn::MIN), @"Ok(Mdn(-3722246))");
        insta::assert_ron_snapshot!(Mdn::try_new(-1), @"Ok(Mdn(-1))");
        insta::assert_ron_snapshot!(Mdn::try_new(0), @"Ok(Mdn(0))");
        insta::assert_ron_snapshot!(Mdn::try_new(1), @"Ok(Mdn(1))");
        insta::assert_ron_snapshot!(Mdn::try_new(Mdn::MAX), @"Ok(Mdn(2261521))");
        insta::assert_ron_snapshot!(Mdn::try_new(Mdn::MAX + 1), @"Err(OutOfMdnRange(2261522))");
    }

    #[test]
    fn t2() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(GregorianYear::MIN, 1, 1), @"Ok(Mdn(-3722246))");
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(2000, 2, 29), @"Ok(Mdn(-1))");
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(2000, 3, 1), @"Ok(Mdn(0))");
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(2000, 3, 2), @"Ok(Mdn(1))");
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(GregorianYear::MAX, 12, 31), @"Ok(Mdn(2261521))");
        Ok(())
    }

    #[test]
    fn t3() -> anyhow::Result<()> {
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(2000, 2, 29)?.to_gymd(), @"(GregorianYear(2000), Month(2), Day(29))");
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(2000, 3, 1)?.to_gymd(), @"(GregorianYear(2000), Month(3), Day(1))");
        insta::assert_ron_snapshot!(Mdn::try_from_gymd_nums(2000, 3, 2)?.to_gymd(), @"(GregorianYear(2000), Month(3), Day(2))");
        Ok(())
    }
}

/*
    QAKtest(mdn_ymd_to_mdn_2, "ymd_to_mdn() 2")
    {
        // gregorian::is_leap_year() and mdn() use different calculations, so we can cross-check them with each other.

        auto verify_year = [](int y) -> void
        {
            // The Mdn of March 1 minus that of February 28 should be 2 for leap years and 1 otherwise.

            int feb28_mdn = ymd_to_mdn(y, 2, 28);
            QAK_verify(is_valid_mdn(feb28_mdn));

            int march1_mdn = ymd_to_mdn(y, 3, 1);
            QAK_verify(is_valid_mdn(march1_mdn));

            int d_days_expected = 1 + gregorian::is_leap_year(y);
            int d_days_actual = march1_mdn - feb28_mdn;

            QAK_verify_equal(d_days_actual, d_days_expected);
        };

        // Start from year +- 1 and work out in both directions so we can debug the nearest errors first.
        int min_year_verified = 0;
        int max_year_verified = 0;
        for (unsigned ui = 0; ui < gregorian::year_max - gregorian::year_min; ++ui)
        {
            bool bc = !(ui & 1); // do bc first
            int y = (ui >> 1) + 1;
            if (bc)
                y = -y;

            verify_year(y);

            if (y < min_year_verified) min_year_verified = y;
            if (max_year_verified < y) max_year_verified = y;
        }
        QAK_verify_equal(min_year_verified, gregorian::year_min);
        QAK_verify_equal(max_year_verified, gregorian::year_max);
    }

    QAKtest(mdn_to_ymd_1, "mdn_to_ymd 1")
    {
        // Check mdn_to_ymd on March 1 for every valid year
        int const y_start = 2000;
        int min_year_verified = 0;
        int max_year_verified = 0;
        unsigned strikeouts = 0;
        for (int ix = 1; true; ++ix)
        {
            int const x = ix >> 1;
            int const y = y_start + ((ix & 1) ? -x : x);

            if (gregorian::is_valid_year(y))
            {
                // For every month in that year.
                for (unsigned m = 1; m <= 12; ++m)
                {
                    // For every day in the month.
                    for (unsigned d = 1; d <= gregorian::cnt_days_in_month(y, m); ++d)
                    {
                        // Test the round trip conversion of (y, m, d).

                        int const mdn = ymd_to_mdn(y, m, d);

                        QAK_verify(is_valid_mdn(mdn));

                        auto const ymd = mdn_to_ymd(mdn);

                        QAK_verify_equal(std::get<0>(ymd), y);
                        QAK_verify_equal(std::get<1>(ymd), m);
                        QAK_verify_equal(std::get<2>(ymd), d);

                        if (2 == d) // Speedup: Skip uninteresting days 3-26 of each month
                            d += 25;
                    }
                }

                strikeouts = 0;
                if (y < min_year_verified) min_year_verified = y;
                if (max_year_verified < y) max_year_verified = y;
            }
            else
            {
                ++strikeouts;
                if (2 <= strikeouts)
                    break;
            }
        }
        QAK_verify_equal(min_year_verified, gregorian::year_min);
        QAK_verify_equal(max_year_verified, gregorian::year_max);
    }

    QAKtest(mdn_to_ymd_2, "mdn_to_ymd 2")
    {
        // For every valid mdn.
        for (int mdn = mdn_min; mdn <= mdn_max; ++mdn)
        {
            // Test the round trip conversion of mdn.

            auto const ymd = mdn_to_ymd(mdn);

            int y = std::get<0>(ymd);
            unsigned m = std::get<1>(ymd);
            unsigned d = std::get<2>(ymd);

            int const mdn2 = ymd_to_mdn(y, m, d);

            QAK_verify_equal(mdn, mdn2);

            if (2 == d) // Speedup: Skip uninteresting days 3-26 of each month
                mdn += 25;
        }
    }

    QAKtest(time_value_ops_mdn_to_tv, "mdn_to_tv()")
    {
        time_value tv = mdn_to_tv(-1);
        QAK_verify_equal( tv.year(),  2000 );
        QAK_verify_equal( tv.month(),    2 );
        QAK_verify_equal( tv.day(),     29 );

        tv = mdn_to_tv(0);
        QAK_verify_equal( tv.year(),  2000 );
        QAK_verify_equal( tv.month(),    3 );
        QAK_verify_equal( tv.day(),      1 );

        tv = mdn_to_tv(1);
        QAK_verify_equal( tv.year(),  2000 );
        QAK_verify_equal( tv.month(),    3 );
        QAK_verify_equal( tv.day(),      2 );
    }

*/
