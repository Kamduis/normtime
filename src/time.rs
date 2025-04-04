//! The measurement of time and date in Normtime.




//=============================================================================
// Crates


use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime, NaiveDateTime, TimeDelta, DateTime};
use thiserror::Error;

use crate::{NORMTIME_OFFSET, DUR_NORMDAY, DUR_NORMMONTH, DUR_NORMYEAR};
use crate::NormTimeDelta;




//=============================================================================
// Errors


#[derive( Error, PartialEq, Debug )]
pub enum TimeError {
	#[error( "Could not parse into NormTime: {0}" )]
	ParseError( String ),

	#[error( transparent )]
	ParseIntError( #[from] std::num::ParseIntError ),
}





//=============================================================================
// Time


/// This struct represents the Normtime.
///
/// The new normtime has its zero-position on 2068-01-01T00:00:00.
/// 1 normday := 100 ks (ca. 1 earth day)
/// 1 normweek := 1 Ms (ca. 12 earth days)
/// 1 normmonth := 3 Ms (ca. 35 earth days)
/// 1 normyear := 30 Ms (ca. 1 earth year, ca. 347 earth days)
#[derive( Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Default )]
pub struct NormTime( i64 );

impl NormTime {
	/// Create a new `NormTime` from Unix `timestamp`.
	///
	/// Returns `None` if the number of seconds would be out of range for a `chrono::NaiveDateTime` (more than ca. 262,000 years away from the zero time).
	pub fn from_timestamp( secs: i64 ) -> Option<Self> {
		let dtime = DateTime::from_timestamp( secs, 0 )?.naive_utc();

		Some( Self::from( dtime ) )
	}

	/// Create a new `NormTime` from `normyear`, `normmonth` and `normday`. `from_ymd( 0, 0, 0 )` represent the 0000-00-00N00:00:00 or the 1st of January 2068 in the common era calendar.
	///
	/// # Arguments
	/// * `normyear` The year in the Normtime calendar.
	/// * `normmonth` The month in the Normtime calendar. 0 is a valid normmonth. But since a normyear has exactly 10 normmonths, this function returns `None` if this argument is grater than 9.
	/// * `normday` The day in the Normtime calendar. 0 is a valid normday. But since a normmonth has exactly 30 normdays, this function returns `None` if this argument is grater than 29.
	pub fn from_ymd_opt( normyear: i32, normmonth: u32, normday: u32 ) -> Option<Self> {
		if normday > 29 || normmonth > 9 {
			return None;
		}

		let seconds = DUR_NORMYEAR * normyear as i64 +
			DUR_NORMMONTH * normmonth as i64 +
			DUR_NORMDAY * normday as i64;

		Some( Self( seconds ) )
	}

	/// Create a new `NormTime` from `self`, adding `hour`, `min` and `sec` to it.
	///
	/// This is not fully identical to the earth wall clock time. `sec` and `min` greater than 60 are allowed as are `hour` greater than 24.
	pub fn and_hms( self, hour: u32, min: u32, sec: u32 ) -> Self {
		let tdelta = TimeDelta::hours( hour as i64 ) + TimeDelta::minutes( min as i64 ) + TimeDelta::seconds( sec as i64 );

		Self( self.0 + tdelta.num_seconds() )
	}

	/// Create a new `NormTime` from `self`, using `normyear` instead of the original normyear.
	pub fn with_year( self, normyear: i32 ) -> Self {
		let secs_of_year_old = self.0.div_euclid( DUR_NORMYEAR ) * DUR_NORMYEAR;
		let secs_of_year_new = normyear as i64 * DUR_NORMYEAR;

		Self( self.0 - secs_of_year_old + secs_of_year_new )
	}

	/// Returns the Unix timestamp representing `self`.
	pub fn timestamp( &self ) -> i64 {
		NORMTIME_OFFSET + self.0
	}

	/// Return the normyear of `self`.
	pub fn year( &self ) -> i32 {
		let year = self.0.div_euclid( DUR_NORMYEAR );

		if year < i32::min_value() as i64 || year > i32::max_value() as i64 {
			panic!( "The year cannot be represented as `i32`" );
		}

		year as i32
	}

	/// Return the string part of `self` as `String`.
	pub fn to_string_year( self ) -> String {
		let year = self.0.div_euclid( DUR_NORMYEAR );

		if year < 0 {
			format!( "-{:0>4}", year.abs() )
		} else {
			format!( "{:0>4}", year )
		}
	}

	/// Return the date part of `self` as `String`.
	pub fn to_string_date( self ) -> String {
		let year = self.0.div_euclid( DUR_NORMYEAR );
		let subyear = self.0.rem_euclid( DUR_NORMYEAR );
		let month = subyear.div_euclid( DUR_NORMMONTH );
		let submonth = subyear.rem_euclid( DUR_NORMMONTH );
		let day = submonth.div_euclid( DUR_NORMDAY );

		if year < 0 {
			format!( "-{:0>4}-{:0>2}-{:0>2}", year.abs(), month, day )
		} else {
			format!( "{:0>4}-{:0>2}-{:0>2}", year, month, day )
		}
	}

	/// Return the date part of `self` as LaTeX command.
	#[cfg( feature = "tex" )]
	pub fn to_latex_date( self ) -> String {
		let mut date_txt = self.to_string_date();

		if date_txt.starts_with( '-' ) {
			date_txt = date_txt.replacen( '-', "−", 1 );
		}

		format!( r"{}\,\uz{{}}", date_txt )
	}

	/// Return the clock part of `self` as `String`.
	pub fn to_string_clock( self ) -> String {
		let subday = self.0.rem_euclid( DUR_NORMDAY );
		let hour = subday.div_euclid( 3600 );
		let subhour = subday.rem_euclid( 3600 );
		let minute = subhour.div_euclid( 60 );
		let seconds = subday.rem_euclid( 60 );

		format!( "{:0>2}:{:0>2}:{:0>2}", hour, minute, seconds )
	}
}

impl PartialEq<NaiveDateTime> for NormTime {
	fn eq( &self, other: &NaiveDateTime ) -> bool {
		( self.0 + NORMTIME_OFFSET ).eq( &other.and_utc().timestamp() )
	}
}

impl Add<NormTimeDelta> for NormTime {
	type Output = Self;

	fn add( self, other: NormTimeDelta ) -> Self::Output {
		Self( self.0 + other.secs )
	}
}

impl Sub for NormTime {
	type Output = NormTimeDelta;

	fn sub( self, other: Self ) -> Self::Output {
		NormTimeDelta::new( self.0 - other.0, 0 ).unwrap()
	}
}

impl fmt::Debug for NormTime {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		write!( f, "{}N{}", self.to_string_date(), self.to_string_clock() )
	}
}

/// Normtime is formatted similar to ISO 8601 but with a "N" instead of a "T" between date and time. Also there is a year 0 as well as there are month and date with the number 0.
///
/// # Example
///
/// ```
/// use normtime::NormTime;
///
/// let d = NormTime::from_ymd_opt( 0, 1, 1).unwrap();
/// assert_eq!( d.to_string(), "0000-01-01N00:00:00" );
///
/// assert_eq!( "+12345-6-7".parse::<NormTime>().unwrap().to_string(), "12345-06-07N00:00:00" );
///
/// assert_eq!( "+12345-6-7N8:9:10".parse::<NormTime>().unwrap().to_string(), "12345-06-07N08:09:10" );
/// ```
impl fmt::Display for NormTime {
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
		fmt::Debug::fmt( self, f )
	}
}

/// Converting `chrono::NaiveDateTime` to `Normtime`.
///
/// # Example
///
/// ```
/// use normtime::NormTime;
/// use chrono::NaiveDate;
///
/// assert_eq!(
///     NormTime::from( NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap().and_hms_opt( 0, 0, 1 ).unwrap() ),
///     NormTime::from_ymd_opt( 0, 0, 0 ).unwrap().and_hms( 0, 0, 1 )
/// );
/// ```
impl From<NaiveDateTime> for NormTime {
	fn from( item: NaiveDateTime ) -> Self {
		Self( item.and_utc().timestamp() - NORMTIME_OFFSET )
	}
}

/// Converting `chrono::NaiveDate` to `Normtime`. The time component is considered to be zero.
///
/// # Example
///
/// ```
/// use normtime::NormTime;
/// use chrono::NaiveDate;
///
/// assert_eq!(
///     NormTime::from( NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap() ),
///     NormTime::from_ymd_opt( 0, 0, 0 ).unwrap().and_hms( 0, 0, 0 )
/// );
/// ```
impl From<NaiveDate> for NormTime {
	fn from( item: NaiveDate ) -> Self {
		Self::from( item.and_time( NaiveTime::from_num_seconds_from_midnight_opt( 0, 0 ).unwrap() ) )
	}
}

/// Converting `Normtime` to `chrono::NaiveDateTime`.
///
/// # Example
///
/// ```
/// use normtime::NormTime;
/// use chrono::{NaiveDate, NaiveDateTime};
///
/// assert_eq!(
///     NaiveDateTime::from( NormTime::from_ymd_opt( 0, 0, 0 ).unwrap().and_hms( 0, 0, 1 ) ),
///     NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap().and_hms_opt( 0, 0, 1 ).unwrap()
/// );
/// ```
impl From<NormTime> for NaiveDateTime {
	fn from( item: NormTime ) -> Self {
		DateTime::from_timestamp( item.timestamp(), 0 ).unwrap().naive_utc()
	}
}

/// Converting `Normtime` to `chrono::NaiveDate`. The `chrono::NaiveDate` does loose all time information that has a resolution finer than one standard day.
///
/// # Example
///
/// ```
/// use normtime::NormTime;
/// use chrono::NaiveDate;
///
/// assert_eq!(
///     NaiveDate::from( NormTime::from_ymd_opt( 0, 0, 0 ).unwrap() ),
///     NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap()
/// );
///
/// assert_eq!(
///     NaiveDate::from( NormTime::from_ymd_opt( 0, 0, 0 ).unwrap().and_hms( 0, 0, 1 ) ),
///     NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap()
/// );
/// ```
impl From<NormTime> for NaiveDate {
	fn from( item: NormTime ) -> Self {
		NaiveDateTime::from( item ).date()
	}
}

/// Parsing a `str` into a `NormTime`. The string must be formatted as `YYYY-M-DD` or `YYYY-M-DDNhh:mm:ss`.
/// * `YYYY` Arbitrary integer number. Can have more or less than four digits, but 4 digits is typical.
/// * `M` Unsigned integer number between 0 and 9. More than one digit is allowed (leading zeros), but untypical.
/// * `DD` Unsigned integer number between 0 and 29. Can have more or less than two digits (leading zeros), but 2 digits is typical.
/// * `hh` Hour
/// * `mm` Minute
/// * `ss` Second
///
/// # Example
///
/// ```
/// use normtime::NormTime;
///
/// let d = NormTime::from_ymd_opt( 900, 3, 12).unwrap();
/// assert_eq!( "0900-03-12".parse::<NormTime>(), Ok( d ) );
///
/// let d = NormTime::from_ymd_opt( 12345, 6, 7 ).unwrap();
/// assert_eq!( "+12345-6-7".parse::<NormTime>(), Ok( d ) );
///
/// let d = NormTime::from_ymd_opt( 12345, 6, 7 ).unwrap().and_hms( 8, 9, 10 );
/// assert_eq!( "+12345-6-7N8:9:10".parse::<NormTime>(), Ok( d ) );
///
/// assert!( "foo".parse::<NormTime>().is_err() );
/// ```
impl FromStr for NormTime {
	type Err = TimeError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let elems: Vec<&str> = s.split( 'N' ).collect();
		if elems.is_empty() || elems.len() > 2 {
			return Err( TimeError::ParseError( s.to_string() ) )
		}

		let elems_date: Vec<&str> = elems[0].split( '-' ).collect();
		if elems_date.len() != 3 {
			return Err( TimeError::ParseError( s.to_string() ) )
		}

		let mut seconds = elems_date[0].parse::<i64>()? * DUR_NORMYEAR;
		seconds += elems_date[1].parse::<i64>()? * DUR_NORMMONTH;
		seconds += elems_date[2].parse::<i64>()? * DUR_NORMDAY;

		let Some( elems_t ) = elems.get( 1 ) else {
			return Ok( NormTime( seconds ) );
		};

		let elems_time: Vec<&str> = elems_t.split( ':' ).collect();
		if elems_time.len() != 3 {
			return Err( TimeError::ParseError( s.to_string() ) )
		}

		seconds += elems_time[0].parse::<i64>()? * 3600;
		seconds += elems_time[1].parse::<i64>()? * 60;
		seconds += elems_time[2].parse::<i64>()?;

		Ok( NormTime( seconds ) )
	}
}


#[cfg( feature = "serde" )]
mod normtime_serde {
	use super::NormTime;

	use std::fmt;

	impl serde::Serialize for NormTime {
		fn serialize<S>( &self, serializer: S ) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			struct FormatWrapped<'a, D: 'a> {
				inner: &'a D,
			}

			impl<'a, D: fmt::Debug> fmt::Display for FormatWrapped<'a, D> {
				fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
					self.inner.fmt( f )
				}
			}

			serializer.collect_str( &FormatWrapped { inner: &self } )
		}
	}

	struct NormTimeVisitor;

	impl<'de> serde::de::Visitor<'de> for NormTimeVisitor {
		type Value = NormTime;

		fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
			formatter.write_str( "a formatted date string" )
		}

		fn visit_str<E>( self, value: &str ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			value.parse().map_err( E::custom )
		}
	}

	impl<'de> serde::Deserialize<'de> for NormTime {
		fn deserialize<D>( deserializer: D ) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			deserializer.deserialize_str( NormTimeVisitor )
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[cfg( feature = "serde" )]
	use serde_test::{Token, assert_tokens};

	#[test]
	fn create_normtime() {
		// Unix-time zero.
		let time_unix_zero = DateTime::from_timestamp( 0, 0 ).unwrap().naive_utc();
		let time_norm_zero = NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap().and_hms_opt( 0, 0, 0 ).unwrap();

		assert_eq!( NormTime::from( time_unix_zero ), time_unix_zero );
		assert_eq!( NormTime::from_timestamp( time_norm_zero.and_utc().timestamp() ).unwrap(), time_norm_zero );
		assert_eq!( NormTime::from_ymd_opt( 0, 0, 0 ).unwrap(), time_norm_zero );
		assert_eq!( NormTime::from_ymd_opt( 1, 0, 0 ).unwrap(), time_norm_zero + TimeDelta::seconds( 30_000_000 ) );
	}

	#[test]
	fn create_normtime_with_year() {
		// Unix-time zero.
		let time_norm_zero = NormTime::from_ymd_opt( 0, 0, 0 ).unwrap();

		assert_eq!( time_norm_zero.with_year( 100 ), time_norm_zero + NormTimeDelta::new_years( 100 ) );
		assert_eq!( time_norm_zero.with_year( 1000 ), time_norm_zero + NormTimeDelta::new_years( 1000 ) );
		assert_eq!( time_norm_zero.with_year( -100 ), time_norm_zero + NormTimeDelta::new_years( -100 ) );
		assert_eq!( time_norm_zero.with_year( time_norm_zero.year() ), time_norm_zero );

		let time_norm_hundred = NormTime::from_ymd_opt( 100, 0, 0 ).unwrap();

		assert_eq!( time_norm_hundred.with_year( 100 ), time_norm_hundred );
		assert_eq!( time_norm_hundred.with_year( 1000 ), NormTime::from_ymd_opt( 1000, 0, 0 ).unwrap() );
		assert_eq!( time_norm_hundred.with_year( -100 ), NormTime::from_ymd_opt( -100, 0, 0 ).unwrap() );
		assert_eq!( time_norm_hundred.with_year( time_norm_hundred.year() ), time_norm_hundred );

	}

	#[test]
	fn normtime_year() {
		assert_eq!( NormTime::from_ymd_opt( 2068, 1, 1 ).unwrap().year(), 2068 );
		assert_eq!( NormTime::from_ymd_opt( 2345, 6, 7 ).unwrap().year(), 2345 );
	}

	#[test]
	fn naive_date_to_normtime() {
		assert_eq!(
			NormTime::from( NaiveDate::from_ymd_opt( 2068, 1, 1 ).unwrap() ),
			NormTime::from_ymd_opt( 0, 0, 0 ).unwrap()
		);
	}

	#[test]
	#[cfg( feature = "serde" )]
	fn test_serialize_deserilaize() {
		// Test that a value serializes to a particular sequence of method calls or deserializes from the sequence of method calls.
		assert_tokens(
			&NormTime::from_ymd_opt( 0, 0, 0 ).unwrap(),
			&[ Token::Str( "0000-00-00N00:00:00" ), ]
		);

		assert_tokens(
			&NormTime::from_ymd_opt( 12345, 6, 7 ).unwrap().and_hms( 8, 9, 10 ),
			&[ Token::Str( "12345-06-07N08:09:10" ), ]
		);
	}
}
