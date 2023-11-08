// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::time::day::Day;

/// Operations on objects that represent a day of the month.
pub trait DayOps {
    /// Extracts the `Day`.
    fn day(&self) -> Day;

    /// The day as a number `1 ..= 31`.
    fn day_as_one_based_u8(&self) -> u8 {
        self.day().0
    }
}
