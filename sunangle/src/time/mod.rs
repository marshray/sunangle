// Copyright 2023 Marsh J. Ray
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod astro_year;
pub mod day;
pub mod day_ops;
pub mod gregorian;
pub mod mdn;
pub mod month;
pub mod month_ops;
//pub mod time_value;
pub mod year_ops;

use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
pub enum Error {
    #[error("invalid Gregorian date")]
    InvalidGregorianYMD { year: i32, month: u8, day: u8 },

    #[error("invalid Gregorian year")]
    InvalidGregorianYear(isize),

    #[error("out of supported range for year")]
    UnsupportedYear(isize),

    #[error("invalid month")]
    InvalidMonth(isize),

    #[error("invalid day")]
    InvalidDay(isize),

    #[error("out of supported range for Mdn")]
    OutOfMdnRange(isize),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
