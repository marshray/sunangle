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
#![allow(clippy::too_many_arguments)]

//? use use std::fmt::Display;
//? use std::ops::RangeInclusive;

//? use anyhow::{anyhow, bail, ensure, Context, Result};
//? use log::{debug, error, info, trace, warn};
//? use serde::{Deserialize, Serialize};

/*
    extern time_value  const leap_seconds[] = { // extern

        // Data from https://en.wikipedia.org/wiki/Leap_second retrieved 2017-08-20.
        time_value::from_ymdhms( 1972,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1972, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1973, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1974, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1975, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1976, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1977, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1978, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1979, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1981,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1982,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1983,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1985,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1987, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1989, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1990, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1992,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1993,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1994,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1995, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 1997,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 1998, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 2005, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 2008, 12, 31, 23, 59, 60),
        time_value::from_ymdhms( 2012,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 2015,  6, 30, 23, 59, 60),
        time_value::from_ymdhms( 2016, 12, 31, 23, 59, 60),
    };

    // Declared extern in time_int.h.
    const time_value s_leap_seconds_known_as_of_y = time_value::from_ym(2017, 8);
*/
