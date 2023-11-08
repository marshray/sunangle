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

use crate::time::month::Month;
use crate::time::year_ops::YearOps;

/// Operations on objects that represent a month.
pub trait MonthOps {
    /// Extracts the `Month`.
    fn month(&self) -> Month;

    /// The month as a number `1 ..= 12`.
    fn month_as_one_based_u8(&self) -> u8 {
        self.month().0
    }

    /// The number of days in the month, but you have to supply a year.
    fn days_in_month(&self, y: &dyn YearOps) -> u8 {
        let m: usize = self.month_as_one_based_u8().into();
        if m == 2 && y.is_leap_year() {
            29
        } else {
            [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][m]
        }
    }
}
