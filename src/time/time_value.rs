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

use crate::time::astro_year::AstroYear;
use crate::time::year_ops::YearOps;
use crate::time::month::Month;
use crate::time::month_ops::MonthOps;
use crate::time::day::Day;
use crate::time::day_ops::DayOps;

// use chrono::{TimeZone, Utc, Tai, Gps};
// use chrono_tz::US::Pacific;
// let pacific_time = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
// let utc_time = pacific_time.with_timezone(&Utc);
// assert_eq!(utc_time, Utc.ymd(1990, 5, 6).and_hms(19, 30, 45));

//pub enum 
//struct Date {
//    ay: AstroYear,
//    m: Month,
//    d: Day,
//}

/*
	//	Type representing time values of flexible resolution.
	//
	//	By definition, time value dates are represented using the Gregorian calendar, even for dates
	//
	|} before its adoption (proleptic).
	//
	//	Leap seconds are representable.
	//
	//	This type does not encode any information about time zone. In the absence of contextual
	//	information specifying otherwise, time_values should be created and interpreted
	//	as UTC.
	//
	struct time_value
	{
		static constexpr int year_min = 1876;
		static constexpr int year_max = 2130;

		//------ Construction from date values.

		static time_value from_y(
			  int year );

		static time_value from_ym(
			  int year
			, unsigned month );

		static time_value from_ymd(
			  int year
			, unsigned month
			, unsigned day );

//#		static time_value from_mdn(int mdn);

		//------ Construction from time values.

		static time_value from_hms(
			  unsigned hour
			, unsigned minute
			, unsigned second );

		//------ Construction from date and time values.

		static time_value from_ymdhms(
			  int year
			, unsigned month
			, unsigned day
			, unsigned hour
			, unsigned minute
			, unsigned second );

		//------

		//	Year, Gregorian.
		//   year_min <= year <= year_min
		int year() const; // Fails if year is not specified.
		qak::optional<int> opt_year() const;
		void set_year(qak::optional<int> opt_year);
		void set_year(int year);

		// 1 <= month <= 12
		unsigned month() const;
		qak::optional<uint32_t> opt_month() const;
		void set_month(qak::optional<uint32_t> opt_month);
		void set_month(unsigned month);

		// 1 <= day <= 31
		// Days-in-month and leap year validity is not enforced by set_day().
		unsigned day() const;
		qak::optional<uint32_t> opt_day() const;
		void set_day(qak::optional<uint32_t> opt_day);
		void set_day(unsigned day);

		// 0 <= hour <= 23
		unsigned hour() const;
		qak::optional<uint32_t> opt_hour() const;
		void set_hour(qak::optional<uint32_t> opt_hour);
		void set_hour(unsigned hour);

		// 0 <= minute <= 59
		unsigned minute() const;
		qak::optional<uint32_t> opt_minute() const;
		void set_minute(qak::optional<uint32_t> opt_minute);
		void set_minute(unsigned minute);

		// 0 <= second <= 61
		unsigned second() const;
		qak::optional<uint32_t> opt_second() const;
		void set_second(qak::optional<uint32_t> opt_second);
		void set_second(unsigned second);

		// 0 <= us < 1e6
		unsigned us() const;
		qak::optional<uint32_t> opt_us() const;
		void set_us(qak::optional<uint32_t> opt_us);
		void set_us(unsigned us);

		//------

		// Verifies that:
		//    All parameters are within expected range.
		//		If February 29th, verifies that any year specified is a leap year.
		//		Checks for correctness of leap seconds.
		bool is_valid() const;

		// Ctors.

		time_value() : data_(0) { }
		time_value(time_value &) = default;
		time_value(time_value &&) = default;
		time_value & operator = (time_value const &) = default;

	private:
		std::uint64_t data_;
	};

	// Returns the count of days in the month.
	//	Fails if month or, if February, month and year are not specified.
	unsigned cnt_days_in_month(time_value tv);

	//	Verify that the year limits meet our expectations:
	// Value 0 is reserved for unspecified.
	//	The value fits in 8 bits.
	// The storage field for year is 10 bits, but the top two bits are reserved for future range expansion.
	static int constexpr year_min = time_value::year_min; // 1876
	static int constexpr year_max = time_value::year_max; // 2130
	static int constexpr yoff = -(year_min - 1); // Added to the API value to get stored value.
	static_assert( year_min + yoff ==   1 );
	static_assert( year_max + yoff == 255 );

	static unsigned constexpr usmax = 1'000'000;

	// Field definitions.
	// Preserve order, used as array indices.
	enum struct field : size_t
	{
		year, month, day, hour, minute, second, us, reserved0
		//, cnt_leap_s
	};

	struct fieldinfo_t
	{
		bool signedness;
		int api_min;         // inclusive
		int api_max;         // inclusive
		int api_offset;
		unsigned stored_min; // inclusive
		unsigned stored_max; // inclusive
		unsigned shift;
		unsigned cnt_bits;
	};

	static fieldinfo_t constexpr fieldinfo[] = {
	   //                api        api      api  stored     stored
	   // signedness     min        max   offset     min        max   shift  cnt_bits
		{     true,  year_min,  year_max,    yoff,      1,       255,     54,       10   }, // year
		{    false,         1,        12,       0,      1,        13,     50,        4   }, // month
		{    false,         1,        31,       0,      1,        32,     45,        5   }, // day
		{    false,         0,        23,       1,      1,        24,     40,        5   }, // hour
		{    false,         0,        59,       1,      1,        60,     34,        6   }, // minute
		{    false,         0,        60,       1,      1,        61,     28,        6   }, // second
		{    false,         0, 1'000'000,       1,      1, 1'000'001,      8,       20   }, // us
		{    false,         0,         0,       0,      0,         0,      6,        8   }, // reserved0
		//{    false,         0,         0,       0,      0,         0,      6,        2   }, // reserved0
		//{    false,         0,        62,       1,      1,        63,      0,        6   }, // cnt_leap_s
	};

	//	Functions for interpreting field defintions, templated on field because signed
	// and unsigned fields need different representation types.

	//---- Operations on specific fields.

	template <field F> struct fieldop
	{
		static bool constexpr signedness = fieldinfo[static_cast<size_t>(F)].signedness;

		// Select the value and optional_value types based on the field's signedness.
		typedef typename std::conditional<signedness, int, unsigned>::type value_type;
		typedef optional<value_type> optional_value_type;

		// The optional_value type should fit within 64 bits.
		static_assert(sizeof(optional_value_type) <= 8);

		// Appropriately-typed short names for the other fieldinfo values.
		static value_type constexpr api_min    = static_cast<value_type>(fieldinfo[static_cast<size_t>(F)].api_min);
		static value_type constexpr api_max    = static_cast<value_type>(fieldinfo[static_cast<size_t>(F)].api_max);
		static value_type constexpr api_offset = static_cast<value_type>(fieldinfo[static_cast<size_t>(F)].api_offset);
		static unsigned   constexpr stored_min = fieldinfo[static_cast<size_t>(F)].stored_min;
		static unsigned   constexpr stored_max = fieldinfo[static_cast<size_t>(F)].stored_max;
		static unsigned   constexpr shift      = fieldinfo[static_cast<size_t>(F)].shift;
		static unsigned   constexpr cnt_bits   = fieldinfo[static_cast<size_t>(F)].cnt_bits;

		// This code commonly usees 'unsigned' and 'int' for handling individual fields.
		static_assert(cnt_bits < (sizeof(int)*8 - 2));

		// The bitmask for the field. (At the low bits, not shifted into place).
		static unsigned constexpr right_mask = (1u << cnt_bits) - 1;

		// The shifted bitmask for the field.
		static uint64_t constexpr shifted_mask = static_cast<uint64_t>(right_mask) << shift;

		// Returns true iff the specified value is within range for the API.
		template <class T>
		static bool api_value_in_range(T api_value)
		{
			return api_min <= api_value && api_value <= api_max;
		}

		// Returns true iff the specified stored value is within range.
		template <class T>
		static bool stored_value_in_range(T stored_value)
		{
			return stored_min <= stored_value && stored_value <= stored_max;
		}

		// Clears the field value.
		// Sets the bit pattern to all 0s.
		static void clear_value(uint64_t & data)
		{
			data &= ~shifted_mask;
		}

		// Sets the field by an optional api value.
		static void set_value(uint64_t & data, optional_value_type opt_api)
		{
			if (opt_api)
				set_value(data, *opt_api);
			else
				clear_value(data);
		}

		// Sets the field by an api value.
		static void set_value(uint64_t & data, value_type api_value)
		{
			fail_unless(api_value_in_range(api_value));
			unsigned stored_value = static_cast<unsigned>(static_cast<int>(api_value) + api_offset);
			assert(stored_value_in_range(stored_value));
			assert(stored_value <= right_mask);
			data &= ~shifted_mask;
			data |= static_cast<uint64_t>(stored_value) << shift;
		}

		// Gets the field api value.
		static optional_value_type get_opt_value(uint64_t data)
		{
			optional_value_type rv;
			unsigned stored_bits = (data >> shift) & right_mask;
			if (stored_bits)
			{
				assert(stored_value_in_range(stored_bits));

				unsigned signextend_shift = sizeof(int)*8 - cnt_bits;
				int i =   signedness
				        ? (static_cast<int>(stored_bits << signextend_shift) >> signextend_shift)
				        : static_cast<int>(stored_bits);

				i -= api_offset;
				assert(api_value_in_range(i));
				rv = static_cast<value_type>(i);
			}
			return rv;
		}
	};

	static_assert(
		64 == (   fieldop<field::year>::cnt_bits
              + fieldop<field::month>::cnt_bits
              + fieldop<field::day>::cnt_bits
              + fieldop<field::hour>::cnt_bits
              + fieldop<field::minute>::cnt_bits
              + fieldop<field::second>::cnt_bits
              + fieldop<field::us>::cnt_bits
              + fieldop<field::reserved0>::cnt_bits
              //+ fieldop<field::cnt_leap_s>::cnt_bits
				  ) );

	//---- year

	int time_value::year() const
	{
		auto opt_year = fieldop<field::year>::get_opt_value(data_);
		fail_unless(opt_year);
		return *opt_year;
	}

	qak::optional<int> time_value::opt_year() const
	{
		return fieldop<field::year>::get_opt_value(data_);
	}

	void time_value::set_year(qak::optional<int32_t> opt_year)
	{
		fieldop<field::year>::set_value(data_, opt_year);
	}

	void time_value::set_year(int year)
	{
		fieldop<field::year>::set_value(data_, year);
	}

	//---- month

	unsigned time_value::month() const
	{
		auto opt_month = fieldop<field::month>::get_opt_value(data_);
		fail_unless(opt_month);
		return *opt_month;
	}

	qak::optional<unsigned> time_value::opt_month() const
	{
		return fieldop<field::month>::get_opt_value(data_);
	}

	void time_value::set_month(qak::optional<uint32_t> opt_month)
	{
		fieldop<field::month>::set_value(data_, opt_month);
	}

	void time_value::set_month(unsigned month)
	{
		fieldop<field::month>::set_value(data_, month);
	}

	//---- day

	unsigned time_value::day() const
	{
		auto opt_day = fieldop<field::day>::get_opt_value(data_);
		fail_unless(opt_day);
		return *opt_day;
	}

	qak::optional<unsigned> time_value::opt_day() const
	{
		return fieldop<field::day>::get_opt_value(data_);
	}

	void time_value::set_day(qak::optional<uint32_t> opt_day)
	{
		fieldop<field::day>::set_value(data_, opt_day);
	}

	void time_value::set_day(unsigned day)
	{
		fieldop<field::day>::set_value(data_, day);
	}

	//---- hour

	unsigned time_value::hour() const
	{
		auto opt_hour = fieldop<field::hour>::get_opt_value(data_);
		fail_unless(opt_hour);
		return *opt_hour;
	}

	qak::optional<unsigned> time_value::opt_hour() const
	{
		return fieldop<field::hour>::get_opt_value(data_);
	}

	void time_value::set_hour(qak::optional<uint32_t> opt_hour)
	{
		fieldop<field::hour>::set_value(data_, opt_hour);
	}

	void time_value::set_hour(unsigned hour)
	{
		fieldop<field::hour>::set_value(data_, hour);
	}

	//---- minute

	unsigned time_value::minute() const
	{
		auto opt_minute = fieldop<field::minute>::get_opt_value(data_);
		fail_unless(opt_minute);
		return *opt_minute;
	}

	qak::optional<unsigned> time_value::opt_minute() const
	{
		return fieldop<field::minute>::get_opt_value(data_);
	}

	void time_value::set_minute(qak::optional<uint32_t> opt_minute)
	{
		fieldop<field::minute>::set_value(data_, opt_minute);
	}

	void time_value::set_minute(unsigned minute)
	{
		fieldop<field::minute>::set_value(data_, minute);
	}

	//---- second

	unsigned time_value::second() const
	{
		auto opt_second = fieldop<field::second>::get_opt_value(data_);
		fail_unless(opt_second);
		return *opt_second;
	}

	qak::optional<unsigned> time_value::opt_second() const
	{
		return fieldop<field::second>::get_opt_value(data_);
	}

	void time_value::set_second(qak::optional<uint32_t> opt_second)
	{
		fieldop<field::second>::set_value(data_, opt_second);
	}

	void time_value::set_second(unsigned second)
	{
		fieldop<field::second>::set_value(data_, second);
	}

	//---- us

	unsigned time_value::us() const
	{
		auto opt_us = fieldop<field::us>::get_opt_value(data_);
		fail_unless(opt_us);
		return *opt_us;
	}

	qak::optional<unsigned> time_value::opt_us() const
	{
		return fieldop<field::us>::get_opt_value(data_);
	}

	void time_value::set_us(qak::optional<uint32_t> opt_us)
	{
		fieldop<field::us>::set_value(data_, opt_us);
	}

	void time_value::set_us(unsigned us)
	{
		fieldop<field::us>::set_value(data_, us);
	}


	//----------------

	time_value time_value::from_y( // static
		  int year )
	{
		time_value tv;
		tv.set_year(year);
		return tv;
	}

	time_value time_value::from_ym( // static
		  int year
		, unsigned month )
	{
		time_value tv;
		tv.set_year(year);
		tv.set_month(month);
		return tv;
	}

	time_value time_value::from_ymd( // static
		  int year
		, unsigned month
		, unsigned day )
	{
		time_value tv;
		tv.set_year(year);
		tv.set_month(month);
		tv.set_day(day);
		return tv;
	}

	time_value time_value::from_hms( // static
		  unsigned hour
		, unsigned minute
		, unsigned second )
	{
		time_value tv;
		tv.set_hour(hour);
		tv.set_minute(minute);
		tv.set_second(second);
		return tv;
	}

	time_value from_ymdhms( // static
		  int year
		, unsigned month
		, unsigned day
		, unsigned hour
		, unsigned minute
		, unsigned second )
	{
		time_value tv;
		tv.set_year(year);
		tv.set_month(month);
		tv.set_day(day);
		tv.set_hour(hour);
		tv.set_minute(minute);
		tv.set_second(second);
		return tv;
	}

	bool time_value::is_valid() const
	{
		auto oy = opt_year();
		if (oy && !fieldop<field::year>::api_value_in_range(*oy)) return false;

		auto om = opt_month();
		if (om && !fieldop<field::month>::api_value_in_range(*om)) return false;

		auto od = opt_day();
		if (od && !fieldop<field::day>::api_value_in_range(*od)) return false;

		auto oh = opt_hour();
		if (oh && !fieldop<field::hour>::api_value_in_range(*oh)) return false;

		auto omi = opt_minute();
		if (omi && !fieldop<field::minute>::api_value_in_range(*omi)) return false;

		auto os = opt_second();
		if (os && !fieldop<field::second>::api_value_in_range(*os)) return false;

		auto ous = opt_us();
		if (ous && !fieldop<field::us>::api_value_in_range(*ous)) return false;

//#		//? TODO can we verify certain historical leap seconds?
//#		if (os && *os == 60)
//#		{
//#			// Attempt to confirm plausibility of leap second
//#			// So far these only happen on 23:59:60 June 30, or Dec 1, UTC.
//#			//	However, time zones could actually put it on another day, hour, and minute.
//#			// So re-evaluate these asserts if they fire inappropriately.
//#			if (omi) assert(59 == *omi);
//#			if (oh) assert(23 == *oh);
//#			if (om && od)
//#				assert(    ( 6 == *om && 30 == *od)
//#			           || (12 == *om && 31 == *od) );
//#		}

		return true;
	}

//#//#//#	bool time_value::is_leap_year() const
//#//#//#	{
//#//#//#		fail_unless(is_active(this, field::year));
//#//#//#		return is_leap_year_known_active(this);
//#//#//#	}
//#//#//#
//#//#//#	int time_value::astronomical_year() const
//#//#//#	{
//#//#//#		fail_unless(is_active(this, field::year));
//#//#//#
//#//#//#		return get_astronomical_year_known_active(this);
//#//#//#	}
//#//#//#
//#//#//#	qak::optional<int> time_value::opt_astronomical_year() const
//#//#//#	{
//#//#//#		return
//#//#//#			  is_active(this, field::year)
//#//#//#			? get_astronomical_year_known_active(this)
//#//#//#			: qak::optional<int>();
//#//#//#	}
//#//#//#
//#//#//#	void time_value::set_astronomical_year(qak::optional<int> opt_astro_year)
//#//#//#	{
//#//#//#		if (opt_astro_year)
//#//#//#			set_astronomical_year(*opt_astro_year);
//#//#//#		else
//#//#//#			clear_field(this, field::year);
//#//#//#	}
//#//#//#
//#//#//#	void time_value::set_astronomical_year(int astro_year)
//#//#//#	{
//#//#//#		set_year(astro_year <= 0 ? astro_year - 1 : astro_year);
//#//#//#	}
//#//#//#
//#//#//#//	void time_value::adjust_years(int years_adj)
//#//#//#//	{
//#//#//#//		if (0 == years_adj)
//#//#//#//			return;
//#//#//#//
//#//#//#//		fail_todo();
//#//#//#//	}
//#//#//#//
//#//#//#//	void time_value::adjust_months(int months_adj)
//#//#//#//	{
//#//#//#//		if (0 == months_adj)
//#//#//#//			return;
//#//#//#//
//#//#//#//		fail_todo();
//#//#//#//	}
//#//#//#
//#//#//#	void time_value::adjust_days(int days_adj)
//#//#//#	{
//#//#//#		if (0 == days_adj)
//#//#//#			return;
//#//#//#
//#//#//#		//? For now, we don't deal with situations where leap seconds could be involved because they could result in
//#//#//#		// an invalid time.
//#//#//#		fail_unless(!is_active(this, field::second));
//#//#//#		fail_unless(!is_active(this, field::us));
//#//#//#
//#//#//#		// Requires all date components.
//#//#//#		fail_unless(is_active(this, field::year));
//#//#//#		fail_unless(is_active(this, field::month));
//#//#//#		fail_unless(is_active(this, field::day));
//#//#//#
//#//#//#		int64_t adj_day = get_field_known_active(this, field::day) + days_adj;
//#//#//#		if (1 <= adj_day && adj_day <= cnt_days_in_month())
//#//#//#		{
//#//#//#			// Simple case, we can just adjust the day value.
//#//#//#			set_field(this, field::day, static_cast<unsigned>(adj_day));
//#//#//#		}
//#//#//#		else // The adjustment crosses a month boundary.
//#//#//#		{
//#//#//#			// Complex case, pivot via mdn.
//#//#//#
//#//#//#			int mdn_adj = mdn(*this) + days_adj;
//#//#//#
//#//#//#			auto tv_adjusted_date = time_value::from_mdn(mdn_adj);
//#//#//#
//#//#//#			//	Make a copy of the current value, apply the adjusted values, and validate it.
//#//#//#			auto tv_copy = *this;
//#//#//#			set_field(&tv_copy, field::year,  get_field_known_active(&tv_adjusted_date, field::year));
//#//#//#			set_field(&tv_copy, field::month, get_field_known_active(&tv_adjusted_date, field::month));
//#//#//#			set_field(&tv_copy, field::day,   get_field_known_active(&tv_adjusted_date, field::day));
//#//#//#
//#//#//#			fail_unless(tv_copy.is_valid());
//#//#//#
//#//#//#			// It worked.
//#//#//#			*this = tv_copy;
//#//#//#		}
//#//#//#	}
//#//#//#
//#//#//#//	void time_value::adjust_hours(int days_adj)
//#//#//#//	{
//#//#//#//		if (0 == days_adj)
//#//#//#//			return;
//#//#//#//
//#//#//#//		fail_todo();
//#//#//#//	}
//#//#//#//
//#//#//#//	void time_value::adjust_minutes(int minutes_adj)
//#//#//#//	{
//#//#//#//		if (0 == minutes_adj)
//#//#//#//			return;
//#//#//#//
//#//#//#//		fail_todo();
//#//#//#//	}
//#//#//#//
//#//#//#//	void time_value::adjust_seconds(int seconds_adj)
//#//#//#//	{
//#//#//#//		if (0 == seconds_adj)
//#//#//#//			return;
//#//#//#//
//#//#//#//		fail_todo();
//#//#//#//	}
//#//#//#//
//#//#//#//	void time_value::adjust_uss(int64_t uss)
//#//#//#//	{
//#//#//#//		fail_unless(is_active(this, field::hour));
//#//#//#//		fail_unless(is_active(this, field::minute));
//#//#//#//		fail_unless(is_active(this, field::second));
//#//#//#//		fail_unless(is_active(this, field::us));
//#//#//#//
//#//#//#//		fail_todo();
//#//#//#//	}
//#//#//#//
//#//#//#//	void time_value::adjust_ymdhmsn(int years, int months, int days, int hours, int minutes, int seconds, int64_t uss)
//#//#//#//	{
//#//#//#//	}
//#//#//#
//#//#//#	static int y_to_ay(int y)
//#//#//#	{
//#//#//#		assert(0 != y);
//#//#//#		return y < 0 ? y + 1 : y;
//#//#//#	}
//#//#//#
//#//#//#	static int ay_to_y(int ay)
//#//#//#	{
//#//#//#		return ay <= 0 ? ay - 1 : ay;
//#//#//#	}
//#//#//#
//#//#//#	time_value time_value::from_mdn(int const mdn) // static
//#//#//#	{
//#//#//#		fail_unless(mdn_min <= mdn && mdn <= mdn_max);
//#//#//#
//#//#//#		time_value tv;
//#//#//#
//#//#//#		if (false)
//#//#//#		{
//#//#//#			fail_todo(); // optimized path
//#//#//#		}
//#//#//#		else // Fallback to binary search.
//#//#//#		{
//#//#//##if 1
//#//#//#			//	Adjust such that March 1, 2000 (the first day of the 400-year cycle) becomes 0.
//#//#//#			int const adj_year_start_mar1 = 305;
//#//#//#			int mdn_adj = mdn + adj_year_start_mar1;
//#//#//#
//#//#//#			//	Find the block of 400 years.
//#//#//#			int b400 = mdn_adj/sc_cnt_days_in_400_years;
//#//#//#			mdn_adj -= b400*sc_cnt_days_in_400_years;
//#//#//#
//#//#//#			// Find the block of 100 years.
//#//#//#			int b100 = mdn_adj/sc_cnt_days_in_most_100_years;
//#//#//#			mdn_adj -= b100*sc_cnt_days_in_most_100_years;
//#//#//#
//#//#//#			// Find the block of 4 years.
//#//#//#			int b4 = mdn_adj/sc_cnt_days_in_most_4_years;
//#//#//#			mdn_adj -= b4*sc_cnt_days_in_most_4_years;
//#//#//#
//#//#//#			// Find the year within the 4 year cycle.
//#//#//#			int b1 = mdn_adj/sc_cnt_days_in_most_1_years;
//#//#//#			mdn_adj -= b1*sc_cnt_days_in_most_1_years;
//#//#//#
//#//#//#			int ay = 2000 + 400*b400 + 100*b100 + 4*b4 + b1;
//#//#//#
//#//#//#			int y = ay_to_y(ay);
//#//#//#
//#//#//#			// mdn_adj 0 -> Mar 1
//#//#//##elif // working
//#//#//#			// Search via astronomical year to avoid complications over year 0.
//#//#//#			int ay = 0;
//#//#//#			for (int ay_l = y_to_ay(time_value::year_min), ay_r = y_to_ay(time_value::year_max); true; )
//#//#//#			{
//#//#//#				fail_unless(ay_l <= ay_r);
//#//#//#
//#//#//#				ay = (ay_l + ay_r)/2;
//#//#//#
//#//#//#				assert(time_value::year_min <= ay && ay <= time_value::year_max);
//#//#//#
//#//#//#				int mdn_y_1_1   = time_value::from_ymd(ay_to_y(ay),  1,  1).mdn();
//#//#//#				int mdn_y_12_31 = time_value::from_ymd(ay_to_y(ay), 12, 31).mdn();
//#//#//#
//#//#//#				if (mdn_y_12_31 < mdn)
//#//#//#					ay_l = ay + 1;
//#//#//#				else if (mdn < mdn_y_1_1)
//#//#//#					ay_r = ay - 1;
//#//#//#				else
//#//#//#					break;
//#//#//#			}
//#//#//#			int y = ay_to_y(ay);
//#//#//##endif
//#//#//#
//#//#//#			assert(time_value::from_ymd(y, 1, 1).mdn() <= mdn && mdn <= time_value::from_ymd(y, 12, 31).mdn());
//#//#//#			tv.set_year(y);
//#//#//#
//#//#//#			// Find month.
//#//#//#
//#//#//#			// Find day.
//#//#//#		}
//#//#//#
//#//#//#		//fail_todo();
//#//#//#		return tv;
//#//#//#	}

	QAKtest(time_value_1, "default construction")
	{
		time_value tv;

		QAK_verify(tv.is_valid());

		QAK_refute(tv.opt_year());
		QAK_refute(tv.opt_month());
		QAK_refute(tv.opt_day());
		QAK_refute(tv.opt_hour());
		QAK_refute(tv.opt_minute());
		QAK_refute(tv.opt_second());
		QAK_refute(tv.opt_us());
	}

	QAKtest(time_value_2, "setting some members")
	{
		time_value tv;
		QAK_verify(tv.is_valid());

		tv.set_year(2017);
		QAK_verify( tv.opt_year() );
		QAK_verify(tv.is_valid());
		QAK_verify_equal( 2017, *tv.opt_year() );
		QAK_verify_equal( 2017, tv.year() );

		tv.set_year(time_value::year_min);
		QAK_verify(tv.opt_year());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(time_value::year_min, *tv.opt_year());

		tv.set_year(time_value::year_max);
		QAK_verify(tv.opt_year());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(time_value::year_max, *tv.opt_year());

		tv.set_month(1);
		QAK_verify(tv.opt_month());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(1, *tv.opt_month());

		tv.set_month(12);
		QAK_verify(tv.opt_month());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(12, *tv.opt_month());

		tv.set_month(qak::optional<uint32_t>());
		QAK_refute(tv.opt_month());

		tv.set_day(31);
		QAK_verify(tv.is_valid());
		QAK_verify(tv.opt_day());
		QAK_verify_equal(31, *tv.opt_day());

		tv.set_day(qak::optional<uint32_t>());
		QAK_refute(tv.opt_day());

		tv.set_hour(23);
		QAK_verify(tv.opt_hour());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(23, *tv.opt_hour());

		tv.set_hour(qak::optional<uint32_t>());
		QAK_refute(tv.opt_hour());

		tv.set_minute(59);
		QAK_verify(tv.opt_minute());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(59, *tv.opt_minute());

		tv.set_minute(qak::optional<uint32_t>());
		QAK_refute(tv.opt_minute());

		tv.set_second(60);
		QAK_verify(tv.opt_second());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(60, *tv.opt_second());

		tv.set_second(qak::optional<uint32_t>());
		QAK_refute(tv.opt_second());

		tv.set_us(999'999);
		QAK_verify(tv.opt_us());
		QAK_verify(tv.is_valid());
		QAK_verify_equal(999'999, *tv.opt_us());

		tv.set_us(qak::optional<uint32_t>());
		QAK_refute(tv.opt_us());
		QAK_verify(tv.is_valid());
	}

*/
