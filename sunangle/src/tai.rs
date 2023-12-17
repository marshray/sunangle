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

use std::borrow::Cow;
use std::convert::TryFrom;

use anyhow::{anyhow, bail, ensure, Context, Result};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct DateTimeTai {
    ndt: NaiveDateTime,
}

impl DateTimeTai {
    #[must_use]
    pub fn from_utc(utc: DateTime<Utc>) -> Self {
        Self::from(utc)
    }

    #[must_use]
    pub fn from_ndt_tai(ndt: NaiveDateTime) -> Self {
        Self { ndt }
    }

    #[must_use]
    pub fn now() -> Self {
        Self::from(Utc::now())
    }

    #[must_use]
    pub fn to_utc(self) -> DateTime<Utc> {
        Into::into(self)
    }

    /// Adds some number of days.
    #[must_use]
    pub fn checked_add_days(self, days: chrono::Days) -> Option<Self> {
        self.ndt.checked_add_days(days).map(Self::from_ndt_tai)
    }

    /// Subtracts some number of days.
    #[must_use]
    pub fn checked_sub_days(self, days: chrono::Days) -> Option<Self> {
        self.ndt.checked_sub_days(days).map(Self::from_ndt_tai)
    }
}

impl std::fmt::Display for DateTimeTai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} TAI", self.ndt)
    }
}

impl std::fmt::Debug for DateTimeTai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl TryFrom<&str> for DateTimeTai {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        debug!("DateTimeTai::try_from('{s}')");

        let s = s.trim();

        let (ndt, s) = NaiveDateTime::parse_and_remainder(s, "%Y-%m-%d %H:%M:%S").map_err(|e| {
            anyhow!("Could not interpret '{s}' as a DateTime in '%Y-%m-%d %H:%M:%S TZ' format: {e}")
        })?;

        let s_pre_lc = s.trim();
        debug!("parsed as NDT: {ndt} followed by '{s_pre_lc}'");

        let s = s_pre_lc.to_lowercase();
        let tai = if s.is_empty() || s == "z" || s == "utc" {
            debug!("TZ is UTC: '{s_pre_lc}'");
            let utc = Utc.from_utc_datetime(&ndt);
            Self::from_utc(utc)
        } else if s == "tai" {
            debug!("TZ is TAI: '{s_pre_lc}'");
            Self::from_ndt_tai(ndt)
        } else {
            bail!("Could not interpret '{s_pre_lc}' as TZ, such as 'UTC' or 'TAI'.")
        };

        Ok(tai)
    }
}

impl std::str::FromStr for DateTimeTai {
    type Err = anyhow::Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<DateTime<Utc>> for DateTimeTai {
    fn from(utc: DateTime<Utc>) -> Self {
        Self {
            ndt: utc
                .naive_utc()
                .checked_add_signed(Duration::seconds(10)) //? TODO actual conversion
                .unwrap_or_else(|| {
                    let new_ndt = NaiveDateTime::MAX;
                    error!("Converting UTC to TAI lost time due to overflow.");
                    new_ndt
                }),
        }
    }
}

impl From<DateTimeTai> for DateTime<Utc> {
    fn from(tai: DateTimeTai) -> Self {
        tai.ndt
            .checked_add_signed(Duration::seconds(-10)) //? TODO actual conversion
            .unwrap_or_else(|| {
                let new_ndt = NaiveDateTime::MIN;
                error!("Converting TAI to UTC gained time due to negative overflow.");
                new_ndt
            })
            .and_utc()
    }
}
