// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// #![allow(dead_code)] //? TODO for development
// #![allow(unused_mut)] //? TODO for development
// #![allow(unused_variables)] //? TODO for development
// #![allow(unused_imports)] //? TODO for development
// #![allow(non_snake_case)] //? TODO for development

use crate::time::astro_year::AstroYear;
use crate::time::gregorian::GregorianYear;

/// Operations on objects that represent a year.
pub trait YearOps {
    /// Extracts the `GregorianYear`.
    fn gregorian_year(&self) -> GregorianYear;

    /// Extracts the `AstroYear`.
    fn astro_year(&self) -> AstroYear;

    /// The year as a `u32` Gregorian year value (no year `0`).
    fn gregorian_year_i32(&self) -> i32;

    /// The year as a `u32` astro year value (year `0` included).
    fn astro_year_i32(&self) -> i32;

    /// Returns whether February has 29 days.
    fn is_leap_year(&self) -> bool;

    /// Returns the number of days in the year.
    fn days_in_year(&self) -> u16 {
        365 + u16::from(self.is_leap_year())
    }
}
