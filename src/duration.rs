//! The measurement of duration or time delta in Normtime.




//=============================================================================
// Crates


use std::iter::Sum;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;

use chrono::TimeDelta;
use thiserror::Error;

#[cfg( feature = "i18n" )] use fluent_templates::Loader;
#[cfg( feature = "i18n" )] use unic_langid::LanguageIdentifier;

#[cfg( feature = "i18n" )] use crate::DisplayLocale;
#[cfg( feature = "tex" )] use crate::{Latex, LatexSym};
#[cfg( all( feature = "i18n", feature = "tex" ) )] use crate::LatexLocale;
#[cfg( feature = "tex" )] use crate::TexOptions;

use crate::{DUR_NORMYEAR, DUR_NORMMONTH, DUR_NORMWEEK, DUR_NORMDAY, DUR_TERRAYEAR, DUR_HOUR, DUR_MINUTE};
#[cfg( feature = "i18n" )] use crate::LOCALES;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum ConversionError {
	#[error( "Cannot parse into `Unit`: {0}" )]
	FromStrFail( String ),
}




//=============================================================================
// Constants


/// The number of nanoseconds per millisecond.
const NANOS_PER_MILLI: i32 = 1_000_000;


/// The number of nanoseconds per second.
const NANOS_PER_SEC: i32 = 1_000_000_000;


/// The number of milliseconds per second.
const MILLIS_PER_SEC: i64 = 1000;




//=============================================================================
// Units


/// Returns the last digit of an unsigned integer number.
#[derive( Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug )]
pub enum Unit {
	Year,
	Month,
	Week,
	Day,
	Hour,
	Minute,
	Second,
}

impl Unit {
	/// Represent unit as symbol.
	///
	/// # Example
	///
	/// ```
	/// use normtime::Unit;
	///
	/// assert_eq!( Unit::Year.to_string_sym(), "y" );
	/// assert_eq!( Unit::Second.to_string_sym(), "s" );
	/// ```
	pub fn to_string_sym( &self ) -> String {
		let res = match self {
			Self::Year => "y",
			Self::Month => "m",
			Self::Week => "w",
			Self::Day => "d",
			Self::Hour => "h",
			Self::Minute => "min",
			Self::Second => "s",
		};

		res.to_string()
	}
}

impl FromStr for Unit {
	type Err = ConversionError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let res = match s.to_lowercase().as_str() {
			"normyears" | "normyear" | "years" | "year" => Self::Year,
			"normmonths" | "normmonth" | "months" | "month" => Self::Month,
			"normweeks" | "normweek" | "weeks" | "week" => Self::Week,
			"normdays" | "normday" | "days" | "day" => Self::Day,
			"hours" | "hour" => Self::Hour,
			"minutes" | "minute" => Self::Minute,
			"seconds" | "second" => Self::Second,
			_ => {
				return Err( ConversionError::FromStrFail( s.to_string() ) );
			},
		};

		Ok( res )
	}
}

/// Representing unit as string.
///
/// # Example
///
/// ```
/// use normtime::Unit;
///
/// assert_eq!( Unit::Year.to_string(), "normyears" );
/// assert_eq!( Unit::Second.to_string(), "seconds" );
/// ```
impl fmt::Display for Unit {
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
		match self {
			Self::Year => write!( f, "normyears" ),
			Self::Month => write!( f, "normmonths" ),
			Self::Week => write!( f, "normweeks" ),
			Self::Day => write!( f, "normdays" ),
			Self::Hour => write!( f, "hours" ),
			Self::Minute => write!( f, "minutes" ),
			Self::Second => write!( f, "seconds" ),
		}
	}
}

#[cfg( feature = "i18n" )]
impl DisplayLocale for Unit {
	/// Representing unit as string, translating the unit into the language specified by `locale`.
	///
	/// # Example
	///
	/// ```
	/// use unic_langid::LanguageIdentifier;
	/// use unic_langid::langid;
	/// use normtime::DisplayLocale;
	/// use normtime::Unit;
	///
	/// const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
	/// const GERMAN: LanguageIdentifier = langid!( "de-DE" );
	///
	/// assert_eq!( Unit::Year.to_string_locale( &US_ENGLISH ), "normyears" );
	/// assert_eq!( Unit::Second.to_string_locale( &US_ENGLISH ), "seconds" );
	/// assert_eq!( Unit::Year.to_string_locale( &GERMAN ), "Normjahre" );
	/// assert_eq!( Unit::Second.to_string_locale( &GERMAN ), "Sekunden" );
	/// ```
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		match self {
			Self::Year =>   format!( "{}", LOCALES.lookup( locale, "normyears" ) ),
			Self::Month =>  format!( "{}", LOCALES.lookup( locale, "normmonths" ) ),
			Self::Week =>   format!( "{}", LOCALES.lookup( locale, "normweeks" ) ),
			Self::Day =>    format!( "{}", LOCALES.lookup( locale, "normdays" ) ),
			Self::Hour =>   format!( "{}", LOCALES.lookup( locale, "hours" ) ),
			Self::Minute => format!( "{}", LOCALES.lookup( locale, "minutes" ) ),
			Self::Second => format!( "{}", LOCALES.lookup( locale, "seconds" ) ),
		}
	}
}

#[cfg( feature = "tex" )]
impl Latex for Unit {}

#[cfg( all( feature = "i18n", feature = "tex" ) )]
impl LatexLocale for Unit {}

#[cfg( feature = "tex" )]
impl LatexSym for Unit {
	/// Represent unit as symbol by using LaTeX unit commands.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{Latex, LatexSym, TexOptions};
	/// use normtime::Unit;
	///
	/// assert_eq!( Unit::Year.to_latex_sym( &TexOptions::new() ), r"\normyear" );
	/// assert_eq!( Unit::Second.to_latex_sym( &TexOptions::new() ), r"\second" );
	/// ```
	fn to_latex_sym( &self, _options: &TexOptions ) -> String {
		let res = match self {
			Self::Year => r"\normyear",
			Self::Month => r"\normmonth",
			Self::Week => r"\normweek",
			Self::Day => r"\normday",
			Self::Hour => r"\hour",
			Self::Minute => r"\minute",
			Self::Second => r"\second",
		};

		res.to_string()
	}
}




//=============================================================================
// Helper functions


/// Returns the last digit of an unsigned integer number.
fn last_digit( number: u64 ) -> u64 {
	if number >= 10 {
		last_digit( number / 10 );
	}

	number % 10
}




//=============================================================================
// Duration

/// Time duration with second precision.
///
/// `NormTimeDelta` differs from e.g. `chrono::TimeDelta`, that it uses normdays, normweeks etc. that have a different duration than standard days etc. The duration of a second is identical, though.
///
/// The range is restricted between `-i64::MAX` and `i64::MAX` *milliseconds*.
#[derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug )]
pub struct NormTimeDelta{
	pub(super) secs: i64,
	nanos: i32,
}

impl NormTimeDelta {
	/// Creates a new `NormTimeDelta` that has a duration of zero seconds.
	pub const ZERO: Self = Self {
		secs: 0,
		nanos: 0,
	};

	/// The minimum possible `NormTimeDelta`. This is limited to `-i64::MAX` milliseconds.
	const MIN: Self = Self {
		secs: -i64::MAX / MILLIS_PER_SEC - 1,
		nanos: NANOS_PER_SEC + ( -i64::MAX % MILLIS_PER_SEC ) as i32 * NANOS_PER_MILLI,
	};

	/// The maximum possible `NormTimeDelta`. This is limited to `i64::MAX` milliseconds.
	const MAX: NormTimeDelta = NormTimeDelta {
		secs: i64::MAX / MILLIS_PER_SEC,
		nanos: ( i64::MAX % MILLIS_PER_SEC ) as i32 * NANOS_PER_MILLI,
	};

	/// Creates a new `NormTimeDelta` that has a duration of `secs` + `nanos`. Returns `None` if the duration is not within `-i64::MAX` and `i64::MAX` *milliseconds* or if `nanos` ≥ 1'000'000'000.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new( 0, 0 ).unwrap(), NormTimeDelta::ZERO );
	/// ```
	pub const fn new( secs: i64, nanos: u32 ) -> Option<Self> {
		if nanos >= 1_000_000_000 ||
			secs < Self::MIN.secs ||
			secs > Self::MAX.secs ||
			( secs == Self::MAX.secs && nanos > Self::MAX.nanos as u32 ) ||
			( secs == Self::MIN.secs && nanos < Self::MIN.nanos as u32 )
		{
			return None;
		}

		Some( Self {
			secs,
			nanos: nanos as i32,
		} )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `secs`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 0 ), NormTimeDelta::ZERO );
	/// ```
	pub fn new_seconds( secs: i64 ) -> Self {
		Self::new( secs, 0 ).expect( "NormTimeDelta::new_seconds is out of bounds" )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `minutes`.
	///
	/// **Note:** If the duration cannot be expressed in `i64` seconds, this method will panic.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_minutes( 0 ), NormTimeDelta::ZERO );
	/// assert_eq!( NormTimeDelta::new_minutes( 1 ), NormTimeDelta::new_seconds( 60 ) );
	/// ```
	pub fn new_minutes( minutes: i64 ) -> Self {
		Self::new( minutes * DUR_MINUTE, 0 ).expect( "NormTimeDelta::new_minutes is out of bounds" )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `hours`.
	///
	/// **Note:** If the duration cannot be expressed in `i64` seconds, this method will panic.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_hours( 0 ), NormTimeDelta::ZERO );
	/// assert_eq!( NormTimeDelta::new_hours( 1 ), NormTimeDelta::new_seconds( 3600 ) );
	/// ```
	pub fn new_hours( hours: i64 ) -> Self {
		Self::new( hours * DUR_HOUR, 0 ).expect( "NormTimeDelta::new_hours is out of bounds" )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `days` normdays.
	///
	/// **Note:** If the duration cannot be expressed in `i64` seconds, this method will panic.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_days( 1 ), NormTimeDelta::new_seconds( 100_000 ) );
	/// ```
	pub fn new_days( days: i64 ) -> Self {
		Self::new( days * DUR_NORMDAY, 0 ).expect( "NormTimeDelta::new_days is out of bounds" )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `years` normyears.
	///
	/// **Note:** If the duration cannot be expressed in `i64` seconds, this method will panic.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ), NormTimeDelta::new_seconds( 30_000_000 ) );
	/// ```
	pub fn new_years( years: i64 ) -> Self {
		Self::new( years * DUR_NORMYEAR, 0 ).expect( "NormTimeDelta::new_years is out of bounds" )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `years` earth years.
	///
	/// **Note:** If the duration cannot be expressed in `i64` seconds, this method will panic.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_earthyears( 1 ), NormTimeDelta::new_seconds( 31_557_600 ) );
	/// ```
	pub fn new_earthyears( years: i64 ) -> Self {
		Self::new( years * DUR_TERRAYEAR, 0 ).expect( "NormTimeDelta::new_earthyears is out of bounds" )
	}

	/// Returns the subsecond fraction of `NormTimeDelta` as number of nanoseconds.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new( 1, 10 ).unwrap().subsec_nanos(), 10 );
	/// ```
	pub fn subsec_nanos( &self ) -> i32 {
		if self.secs < 0 && self.nanos > 0 {
			self.nanos - NANOS_PER_SEC
		} else {
			self.nanos
		}
	}

	/// Computes the absolute value of `self`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( -1 ).abs(), NormTimeDelta::new_seconds( 30_000_000 ) );
	/// ```
	pub fn abs( self ) -> Self {
		if self.secs < 0 && self.nanos != 0 {
			Self {
				secs: ( self.secs + 1 ).abs(),
				nanos: NANOS_PER_SEC - self.nanos,
			}
		} else {
			Self {
				secs: self.secs.abs(),
				nanos: self.nanos
			}
		}
	}

	/// Returns `true` if `self` has a duration of 0 seconds.
	pub fn is_zero( &self ) -> bool {
		self.secs == 0 && self.nanos == 0
	}

	/// Returns the duration of `self` in seconds.
	pub fn seconds( &self ) -> i64 {
		if self.secs < 0 && self.nanos > 0 {
			self.secs + 1
		} else {
			self.secs
		}
	}

	/// Returns the duration of `self` in minutes.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ).minutes(), 500_000 );
	/// assert_eq!( NormTimeDelta::new_seconds( 60 ).minutes(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 119 ).minutes(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 120 ).minutes(), 2 );
	/// ```
	pub fn minutes( &self ) -> i64 {
		self.seconds() / DUR_MINUTE
	}

	/// Returns the duration of `self` in hours.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ).hours(), 8333 );
	/// assert_eq!( NormTimeDelta::new_seconds( 3600 ).hours(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 7199 ).hours(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 7200 ).hours(), 2 );
	/// ```
	pub fn hours( &self ) -> i64 {
		self.seconds() / DUR_HOUR
	}

	/// Returns the duration of `self` in normdays.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ).days(), 300 );
	/// assert_eq!( NormTimeDelta::new_seconds( 100_000 ).days(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 199_999 ).days(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 200_000 ).days(), 2 );
	/// ```
	pub fn days( &self ) -> i64 {
		self.seconds() / DUR_NORMDAY
	}

	/// Returns the duration of `self` in normweeks.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ).weeks(), 30 );
	/// assert_eq!( NormTimeDelta::new_seconds( 1_000_000 ).weeks(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 1_999_999 ).weeks(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 2_000_000 ).weeks(), 2 );
	/// ```
	pub fn weeks( &self ) -> i64 {
		self.seconds() / DUR_NORMWEEK
	}

	/// Returns the duration of `self` in normmonths.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ).months(), 10 );
	/// assert_eq!( NormTimeDelta::new_seconds( 3_000_000 ).months(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 3_999_999 ).months(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 6_000_000 ).months(), 2 );
	/// ```
	pub fn months( &self ) -> i64 {
		self.seconds() / DUR_NORMMONTH
	}

	/// Returns the duration of `self` in normyears.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ).years(), 1 );
	/// assert_eq!( NormTimeDelta::new_seconds( 60_000_000 ).years(), 2 );
	/// assert_eq!( NormTimeDelta::new_seconds( 89_000_000 ).years(), 2 );
	/// assert_eq!( NormTimeDelta::new_seconds( 90_000_000 ).years(), 3 );
	/// ```
	pub fn years( &self ) -> i64 {
		self.seconds() / DUR_NORMYEAR
	}

	/// Returns the duration of `self` in rough categories. E.g. "Kleinkind", "Kind", "Teenager", "Anfang 20", "Mitte 20", "Ende 20" etc.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 2 ).roughly( false ), "Kleinkind" );
	/// assert_eq!( NormTimeDelta::new_years( 4 ).roughly( false ), "Kind" );
	/// assert_eq!( NormTimeDelta::new_years( 13 ).roughly( false ), "Teenager" );
	/// assert_eq!( NormTimeDelta::new_years( 20 ).roughly( false ), "Anfang 20" );
	/// assert_eq!( NormTimeDelta::new_years( 24 ).roughly( false ), "Mitte 20" );
	/// assert_eq!( NormTimeDelta::new_years( 28 ).roughly( false ), "Ende 20" );
	///
	/// assert_eq!( NormTimeDelta::new_years( 2 ).roughly( true ), "Sehr jung" );
	/// assert_eq!( NormTimeDelta::new_years( 4 ).roughly( true ), "Jung" );
	/// assert_eq!( NormTimeDelta::new_years( 13 ).roughly( true ), "An Reife gewonnen" );
	/// assert_eq!( NormTimeDelta::new_years( 20 ).roughly( true ), "Anfang 20" );
	/// ```
	pub fn roughly( &self, generic: bool ) -> String {
		let number = self.years();

		match number {
			i64::MIN..=-1 => "Ungeboren".to_string(),
			0..=2 => if generic { "Sehr jung".to_string() } else { "Kleinkind".to_string() },
			3..=12 => if generic { "Jung".to_string() } else { "Kind".to_string() },
			13..=19 => if generic { "An Reife gewonnen".to_string() } else { "Teenager".to_string() },
			_ => {
				let tens = ( number / 10 ) * 10;
				match last_digit( number as u64 ) {
					0..=2 => format!( "Anfang {}", tens ),
					3..=6 => format!( "Mitte {}", tens ),
					7..=9 => format!( "Ende {}", tens ),
					_ => unreachable!(),
				}
			},
		}
	}

	/// Returns duration as a vector of unit representations with selectable units rounded to the smallest unit provided.
	fn as_units( &self, units: &[Unit] ) -> Vec<(i64, Unit)> {
		let mut number = self.seconds();

		let mut elems: Vec<(i64, Unit)> = Vec::new();

		if units.iter().any( |x| x == &Unit::Year ) {
			let val = number / DUR_NORMYEAR;
			elems.push( ( val, Unit::Year ) );
			number -= val * DUR_NORMYEAR;
		}
		if units.iter().any( |x| x == &Unit::Month ) {
			let val = number / DUR_NORMMONTH;
			elems.push( ( val, Unit::Month ) );
			number -= val * DUR_NORMMONTH;
		}
		if units.iter().any( |x| x == &Unit::Week ) {
			let val = number / DUR_NORMWEEK;
			elems.push( ( val, Unit::Week ) );
			number -= val * DUR_NORMWEEK;
		}
		if units.iter().any( |x| x == &Unit::Day ) {
			let val = number / DUR_NORMDAY;
			elems.push( ( val, Unit::Day ) );
			number -= val * DUR_NORMDAY;
		}
		if units.iter().any( |x| x == &Unit::Hour ) {
			let val = number / DUR_HOUR;
			elems.push( ( val, Unit::Hour ) );
			number -= val * DUR_HOUR;
		}
		if units.iter().any( |x| x == &Unit::Minute ) {
			let val = number / DUR_MINUTE;
			elems.push( ( val, Unit::Minute ) );
			number -= val * DUR_MINUTE;
		}
		if units.iter().any( |x| x == &Unit::Second ) {
			elems.push( ( number, Unit::Second ) );
		}

		elems
	}

	/// Returns the duration as string with symbol as unit.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 1 ).to_string_sym(), "1 s" );
	/// assert_eq!( NormTimeDelta::new_seconds( 10 ).to_string_sym(), "10 s" );
	/// ```
	pub fn to_string_sym( &self ) -> String {
		format!( r"{} s", self.secs )
	}

	/// Returns a string representation of `self` with selectable units rounded to the smallest unit provided. Selected units, that are too large (would be 0) are omitted.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// let delta = NormTimeDelta::new_seconds( 90_005_000 );
	/// assert_eq!( delta.to_string_unit( &[ Unit::Day ] ), "900 normdays" );
	/// assert_eq!( delta.to_string_unit( &[ Unit::Day, Unit::Hour ] ), "900 normdays 1 hour" );
	/// assert_eq!(
	///     delta.to_string_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     "900 normdays 1 hour 23 minutes"
	/// );
	///
	/// let delta_1 = NormTimeDelta::new_seconds( 5_000 );
	/// assert_eq!( delta_1.to_string_unit( &[ Unit::Day, Unit::Hour ] ), "1 hour" );
	/// assert_eq!(
	///     delta_1.to_string_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     "1 hour 23 minutes"
	/// );
	/// ```
	pub fn to_string_unit( &self, units: &[Unit] ) -> String {
		self.as_units( units ).iter()
			.filter( |( k, _ )| k > &0 )
			.map( |( k, v )| {
				let name_unit = v.to_string();
				let postfix = if *k == 1 {
					name_unit[0..name_unit.len()-1].to_string()
				} else {
					name_unit
				};
				format!( "{} {}", k, postfix )
			} )
			.collect::<Vec<String>>()
			.join( " " )
	}

	/// Returns a string representation of `self` with selectable units rounded to the smallest unit provided. Selected units, that are too large (would be 0) are omitted.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// let delta = NormTimeDelta::new_seconds( 90_005_000 );
	/// assert_eq!( delta.to_latex_unit( &[ Unit::Day ] ), "900~normdays" );
	/// assert_eq!( delta.to_latex_unit( &[ Unit::Day, Unit::Hour ] ), "900~normdays 1~hour" );
	/// assert_eq!(
	///     delta.to_latex_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     "900~normdays 1~hour 23~minutes"
	/// );
	///
	/// let delta_1 = NormTimeDelta::new_seconds( 5_000 );
	/// assert_eq!( delta_1.to_latex_unit( &[ Unit::Day, Unit::Hour ] ), "1~hour" );
	/// assert_eq!(
	///     delta_1.to_latex_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     "1~hour 23~minutes"
	/// );
	/// ```
	#[cfg( feature = "tex" )]
	pub fn to_latex_unit( &self, units: &[Unit] ) -> String {
		self.as_units( units ).iter()
			.filter( |( k, _ )| k > &0 )
			.map( |( k, v )| {
				let name_unit = v.to_string();
				let postfix = if *k == 1 {
					name_unit[0..name_unit.len()-1].to_string()
				} else {
					name_unit
				};
				format!( "{}~{}", k, postfix )
			} )
			.collect::<Vec<String>>()
			.join( " " )
	}

	/// Returns a string representation of `self` with selectable units rounded to the smallest unit provided. Selected units, that are too large (would be 0) are omitted. The string is using the language that is provided by `locale`.
	///
	/// # Example
	///
	/// ```
	/// use unic_langid::LanguageIdentifier;
	/// use unic_langid::langid;
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
	/// const GERMAN: LanguageIdentifier = langid!( "de-DE" );
	///
	/// let delta = NormTimeDelta::new_seconds( 90_005_000 );
	/// assert_eq!( delta.to_string_unit_locale( &[ Unit::Day ], &US_ENGLISH ), "900 normdays" );
	/// assert_eq!(
	///     delta.to_string_unit_locale( &[ Unit::Day, Unit::Hour ], &US_ENGLISH ),
	///     "900 normdays 1 hour"
	/// );
	/// assert_eq!(
	///     delta.to_string_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &US_ENGLISH ),
	///     "900 normdays 1 hour 23 minutes"
	/// );
	/// assert_eq!( delta.to_string_unit_locale( &[ Unit::Day ], &GERMAN ), "900 Normtage" );
	/// assert_eq!(
	///     delta.to_string_unit_locale( &[ Unit::Day, Unit::Hour ], &GERMAN ),
	///     "900 Normtage 1 Stunde"
	/// );
	/// assert_eq!(
	///     delta.to_string_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &GERMAN ),
	///     "900 Normtage 1 Stunde 23 Minuten"
	/// );
	///
	/// let delta_1 = NormTimeDelta::new_seconds( 5_000 );
	/// assert_eq!(
	///     delta_1.to_string_unit_locale( &[ Unit::Day, Unit::Hour ], &US_ENGLISH ),
	///     "1 hour"
	/// );
	/// assert_eq!(
	///     delta_1.to_string_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &US_ENGLISH ),
	///     "1 hour 23 minutes"
	/// );
	/// assert_eq!(
	///     delta_1.to_string_unit_locale( &[ Unit::Day, Unit::Hour ], &GERMAN ),
	///     "1 Stunde"
	/// );
	/// assert_eq!(
	///     delta_1.to_string_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &GERMAN ),
	///     "1 Stunde 23 Minuten"
	/// );
	/// ```
	#[cfg( feature = "i18n" )]
	pub fn to_string_unit_locale( &self, units: &[Unit], locale: &LanguageIdentifier ) -> String {
		self.as_units( units ).iter()
			.filter( |( k, _ )| k > &0 )
			.map( |( k, v )| {
				let name_unit = v.to_string_locale( locale );
				let postfix = if *k == 1 {
					name_unit[0..name_unit.len()-1].to_string()
				} else {
					name_unit
				};
				format!( "{} {}", k, postfix )
			} )
			.collect::<Vec<String>>()
			.join( " " )
	}

	/// Returns a string representation of `self` with selectable units rounded to the smallest unit provided. Selected units, that are too large (would be 0) are omitted. This string is intended to be used by LaTeX. The string is translated into the language that is provided by `locale`.
	///
	/// # Example
	///
	/// ```
	/// use unic_langid::LanguageIdentifier;
	/// use unic_langid::langid;
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
	/// const GERMAN: LanguageIdentifier = langid!( "de-DE" );
	///
	/// let delta = NormTimeDelta::new_seconds( 90_005_000 );
	/// assert_eq!( delta.to_latex_unit_locale( &[ Unit::Day ], &US_ENGLISH ), "900~normdays" );
	/// assert_eq!(
	///     delta.to_latex_unit_locale( &[ Unit::Day, Unit::Hour ], &US_ENGLISH ),
	///     "900~normdays 1~hour"
	/// );
	/// assert_eq!(
	///     delta.to_latex_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &US_ENGLISH ),
	///     "900~normdays 1~hour 23~minutes"
	/// );
	/// assert_eq!( delta.to_latex_unit_locale( &[ Unit::Day ], &GERMAN ), "900~Normtage" );
	/// assert_eq!(
	///     delta.to_latex_unit_locale( &[ Unit::Day, Unit::Hour ], &GERMAN ),
	///     "900~Normtage 1~Stunde"
	/// );
	/// assert_eq!(
	///     delta.to_latex_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &GERMAN ),
	///     "900~Normtage 1~Stunde 23~Minuten"
	/// );
	///
	/// let delta_1 = NormTimeDelta::new_seconds( 5_000 );
	/// assert_eq!(
	///     delta_1.to_latex_unit_locale( &[ Unit::Day, Unit::Hour ], &US_ENGLISH ),
	///     "1~hour"
	/// );
	/// assert_eq!(
	///     delta_1.to_latex_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &US_ENGLISH ),
	///     "1~hour 23~minutes"
	/// );
	/// assert_eq!(
	///     delta_1.to_latex_unit_locale( &[ Unit::Day, Unit::Hour ], &GERMAN ),
	///     "1~Stunde"
	/// );
	/// assert_eq!(
	///     delta_1.to_latex_unit_locale( &[ Unit::Day, Unit::Hour, Unit::Minute ], &GERMAN ),
	///     "1~Stunde 23~Minuten"
	/// );
	/// ```
	#[cfg( all( feature = "i18n", feature = "tex" ) )]
	pub fn to_latex_unit_locale( &self, units: &[Unit], locale: &LanguageIdentifier ) -> String {
		self.as_units( units ).iter()
			.filter( |( k, _ )| k > &0 )
			.map( |( k, v )| {
				let name_unit = v.to_string_locale( locale );
				let postfix = if *k == 1 {
					name_unit[0..name_unit.len()-1].to_string()
				} else {
					name_unit
				};
				format!( "{}~{}", k, postfix )
			} )
			.collect::<Vec<String>>()
			.join( " " )
	}

	/// Returns a string representation of `self` with selectable units rounded to the smallest unit provided. The units are expressed as symbols.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// let delta = NormTimeDelta::new_seconds( 90_005_000 );
	/// assert_eq!( delta.to_string_sym_unit( &[ Unit::Day ] ), "900 d" );
	/// assert_eq!( delta.to_string_sym_unit( &[ Unit::Day, Unit::Hour ] ), "900 d 1 h" );
	/// assert_eq!(
	///     delta.to_string_sym_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     "900 d 1 h 23 min"
	/// );
	///
	/// let delta_1 = NormTimeDelta::new_seconds( 5_000 );
	/// assert_eq!( delta_1.to_string_sym_unit( &[ Unit::Day, Unit::Hour ] ), "1 h" );
	/// assert_eq!(
	///     delta_1.to_string_sym_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     "1 h 23 min"
	/// );
	/// ```
	pub fn to_string_sym_unit( &self, units: &[Unit] ) -> String {
		self.as_units( units ).iter()
			.filter( |( k, _ )| k > &0 )
			.map( |( k, v )| format!( "{} {}", k, v.to_string_sym() ) )
			.collect::<Vec<String>>()
			.join( " " )
	}

	/// Returns a LaTeX-string representation of `self` with selectable units rounded to the smallest unit provided. The units are expressed as symbols using the LaTeX `{siunitx}` package.
	///
	/// This method is only available when the **tex** feature has been activated.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// let delta = NormTimeDelta::new_seconds( 90_005_000 );
	/// assert_eq!(
	///     delta.to_latex_sym_unit( &[ Unit::Day ] ),
	///     r"\qty{900}{\normday}"
	/// );
	/// assert_eq!(
	///     delta.to_latex_sym_unit( &[ Unit::Day, Unit::Hour ] ),
	///     r"\qty{900}{\normday}\,\qty{1}{\hour}"
	/// );
	/// assert_eq!(
	///     delta.to_latex_sym_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     r"\qty{900}{\normday}\,\qty{1}{\hour}\,\qty{23}{\minute}"
	/// );
	///
	/// let delta_1 = NormTimeDelta::new_seconds( 5_000 );
	/// assert_eq!(
	///     delta_1.to_latex_sym_unit( &[ Unit::Day, Unit::Hour ] ),
	///     r"\qty{1}{\hour}"
	/// );
	/// assert_eq!(
	///     delta_1.to_latex_sym_unit( &[ Unit::Day, Unit::Hour, Unit::Minute ] ),
	///     r"\qty{1}{\hour}\,\qty{23}{\minute}"
	/// );
	/// ```
	#[cfg( feature = "tex" )]
	pub fn to_latex_sym_unit( &self, units: &[Unit] ) -> String {
		self.as_units( units ).iter()
			.filter( |( k, _ )| k > &0 )
			.map( |( k, v )| format!( r"\qty{{{}}}{{{}}}", k, v.to_latex_sym( &TexOptions::new() ) ) )
			.collect::<Vec<String>>()
			.join( "\\," )
	}

	/// Adding two `NormTimeDelta`s. If an overflow occurs, this function returns `None`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 1 ).checked_add( &NormTimeDelta::new_seconds( 10 ) ),
	///     Some( NormTimeDelta::new_seconds( 11 ) )
	/// );
	/// assert!(
	///     NormTimeDelta::new_seconds( i64::MAX / 1000 ).checked_add( &NormTimeDelta::new_seconds( 1 ) ).is_none()
	/// );
	/// ```
	#[must_use]
	pub const fn checked_add( &self, rhs: &Self ) -> Option<Self> {
		// Since the maximum number of seconds in `NormTimeDelta` is limited to `i64::MAX *milliseconds*, the addition of two `NormTimeDelta`s can never overflow an `i64`.
		// The check if a valid `NormTimeDelta` can be created is happening in `new()`.
		let mut secs = self.secs + rhs.secs;
		let mut nanos = self.nanos + rhs.nanos;

		if nanos >= NANOS_PER_SEC {
			nanos -= NANOS_PER_SEC;
			secs += 1;
		}

		Self::new( secs, nanos as u32 )
	}

    /// Subtracting two `NormTimeDelta`s. If an overflow occurs, this function returns `None`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 1 ).checked_sub( &NormTimeDelta::new_seconds( 10 ) ),
	///     Some( NormTimeDelta::new_seconds( -9 ) )
	/// );
	/// assert!(
	///     NormTimeDelta::new_seconds( -i64::MAX / 1000 ).checked_sub( &NormTimeDelta::new_seconds( 1 ) ).is_none()
	/// );
	/// ```
	#[must_use]
	pub const fn checked_sub( &self, rhs: &Self ) -> Option<Self> {
		// Since the minimum number of seconds in `NormTimeDelta` is limited to `-i64::MAX *milliseconds*, the subtraction of two `NormTimeDelta`s can never overflow an `i64`.
		// The check if a valid `NormTimeDelta` can be created is happening in `new()`.
		let mut secs = self.secs - rhs.secs;
		let mut nanos = self.nanos - rhs.nanos;

		if nanos < 0 {
			nanos += NANOS_PER_SEC;
			secs -= 1;
		}

		Self::new( secs, nanos as u32 )
	}

	/// Multiply a `NormTimeDelta` with a `i32`. If an overflow occurs, this function returns `None`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 1 ).checked_mul( 2 ), Some( NormTimeDelta::new_seconds( 2 ) ) );
	/// assert!( NormTimeDelta::new_seconds( i64::MAX / 1000 ).checked_mul( 1001 ).is_none() );
	/// ```
	#[must_use]
	pub const fn checked_mul( &self, rhs: i32 ) -> Option<Self> {
		// Multiply nanoseconds as `i64` to prevent overflow.
		let nanos_total = self.nanos as i64 * rhs as i64;

		let secs_more = nanos_total.div_euclid( NANOS_PER_SEC as i64 );
		let nanos = nanos_total.rem_euclid( NANOS_PER_SEC as i64 );

		// Multiply seconds as `i128` to prevent overflow
		let secs: i128 = self.secs as i128 * rhs as i128 + secs_more as i128;

		if secs <= i64::MIN as i128 || secs >= i64::MAX as i128 {
			return None;
		};

		Some( Self {
			secs: secs as i64,
			nanos: nanos as i32
		} )
	}

	/// Divide a `NormTimeDelta` with a `i32`. A division by 0 returns `None`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 2 ).checked_div( 2 ), Some( NormTimeDelta::new_seconds( 1 ) ) );
	/// assert!( NormTimeDelta::new_seconds( 2 ).checked_div( 0 ).is_none() );
	/// ```
	#[must_use]
	pub const fn checked_div( &self, rhs: i32 ) -> Option<Self> {
		if rhs == 0 {
			return None;
		}

		let secs = self.secs / rhs as i64;
		let carry = self.secs % rhs as i64;
		let nanos_more = carry * NANOS_PER_SEC as i64 / rhs as i64;
		let nanos = self.nanos / rhs + nanos_more as i32;

		let ( secs, nanos ) = match nanos {
			i32::MIN..=-1 => ( secs - 1, nanos + NANOS_PER_SEC ),
			NANOS_PER_SEC..=i32::MAX => ( secs + 1, nanos - NANOS_PER_SEC ),
			_ => ( secs, nanos ),
		};

		Some( Self { secs, nanos } )
	}
}


impl Add for NormTimeDelta {
	type Output = Self;

	fn add( self, rhs: Self ) -> Self::Output {
		self.checked_add( &rhs ).expect( "Overflow in `NormTimeDelta + NormTimeDelta`" )
	}
}


impl Sub for NormTimeDelta {
	type Output = Self;

	fn sub( self, rhs: Self ) -> Self {
		self.checked_sub( &rhs ).expect( "Overflow in `NormTimeDelta - NormTimeDelta`")
	}
}


impl Mul<i32> for NormTimeDelta {
	type Output = Self;

	fn mul(self, rhs: i32) -> Self {
		self.checked_mul(rhs).expect("Overflow in `NormTimeDelta * i32`")
	}
}


impl Div<i32> for NormTimeDelta {
	type Output = Self;

	fn div( self, rhs: i32 ) -> Self {
		self.checked_div( rhs ).expect( "Division by 0" )
	}
}


impl<'a> Sum<&'a NormTimeDelta> for NormTimeDelta {
	fn sum<I: Iterator<Item = &'a NormTimeDelta>>( iter: I ) -> Self {
		iter.fold( NormTimeDelta::ZERO, |acc, x| acc + *x )
	}
}

impl Sum<NormTimeDelta> for NormTimeDelta {
	fn sum<I: Iterator<Item = NormTimeDelta>>( iter: I ) -> Self {
		iter.fold( NormTimeDelta::ZERO, |acc, x| acc + x )
	}
}

impl From<TimeDelta> for NormTimeDelta {
	fn from( item: TimeDelta ) -> Self {
		Self {
			secs: item.num_seconds(),
			nanos: item.subsec_nanos(),
		}
	}
}

/// Normtime duration is displayed in seconds.
///
/// # Example
///
/// ```
/// use normtime::NormTimeDelta;
///
/// assert_eq!( NormTimeDelta::new_seconds( 100 ).to_string(), "100 seconds" );
/// assert_eq!( NormTimeDelta::new_days( 1 ).to_string(), "100000 seconds" );
/// ```
impl fmt::Display for NormTimeDelta {
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
		match self.secs {
			1 => write!( f, "{} second", self.secs ),
			_ => write!( f, "{} seconds", self.secs ),
		}
	}
}

#[cfg( feature = "i18n" )]
impl DisplayLocale for NormTimeDelta {
	/// Returns a string providing the duration of `self` in seconds translated to the language provided by `locale`.
	///
	/// # Example
	///
	/// ```
	/// use unic_langid::LanguageIdentifier;
	/// use unic_langid::langid;
	/// use normtime::DisplayLocale;
	/// use normtime::{NormTimeDelta, Unit};
	///
	/// const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
	/// const GERMAN: LanguageIdentifier = langid!( "de-DE" );
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 1 ).to_string_locale( &US_ENGLISH ), "1 second" );
	/// assert_eq!( NormTimeDelta::new_seconds( 10 ).to_string_locale( &US_ENGLISH ), "10 seconds" );
	/// assert_eq!( NormTimeDelta::new_seconds( 1 ).to_string_locale( &GERMAN ), "1 Sekunde" );
	/// assert_eq!( NormTimeDelta::new_seconds( 10 ).to_string_locale( &GERMAN ), "10 Sekunden" );
	/// ```
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		match self.secs {
			1 => format!( "{} {}", self.secs, LOCALES.lookup( locale, "second" ) ),
			_ => format!( "{} {}", self.secs, LOCALES.lookup( locale, "seconds" ) ),
		}
	}
}

#[cfg( feature = "tex" )]
impl Latex for NormTimeDelta {
	/// Returning `self` as LaTeX string.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{Latex, TexOptions};
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 1 ).to_latex( &TexOptions::new() ), "1~second" );
	/// assert_eq!( NormTimeDelta::new_seconds( 100 ).to_latex( &TexOptions::new() ), "100~seconds" );
	/// assert_eq!( NormTimeDelta::new_days( 1 ).to_latex( &TexOptions::new() ), "100000~seconds" );
	/// ```
	fn to_latex( &self, _options: &TexOptions ) -> String {
		match self.secs {
			1 => format!( "{}~second", self.secs ),
			_ => format!( "{}~seconds", self.secs ),
		}
	}
}

#[cfg( all( feature = "i18n", feature = "tex" ) )]
impl LatexLocale for NormTimeDelta {
	/// Returns a string providing the duration of `self` in seconds translated to the language provided by `locale`.
	///
	/// # Example
	///
	/// ```
	/// use unic_langid::LanguageIdentifier;
	/// use unic_langid::langid;
	/// use normtime::{Latex, LatexLocale, TexOptions};
	/// use normtime::NormTimeDelta;
	///
	/// const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
	/// const GERMAN: LanguageIdentifier = langid!( "de-DE" );
	///
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 1 ).to_latex_locale( &US_ENGLISH, &TexOptions::new() ),
	///     "1~second"
	/// );
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 10 ).to_latex_locale( &US_ENGLISH, &TexOptions::new() ),
	///     "10~seconds"
	/// );
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 1 ).to_latex_locale( &GERMAN, &TexOptions::new() ),
	///     "1~Sekunde"
	/// );
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 10 ).to_latex_locale( &GERMAN, &TexOptions::new() ),
	///     "10~Sekunden"
	/// );
	/// ```
	fn to_latex_locale( &self, locale: &LanguageIdentifier, _options: &TexOptions ) -> String {
		match self.secs {
			1 => format!( "{}~{}", self.secs, LOCALES.lookup( locale, "second" ) ),
			_ => format!( "{}~{}", self.secs, LOCALES.lookup( locale, "seconds" ) ),
		}
	}
}

#[cfg( feature = "tex" )]
impl LatexSym for NormTimeDelta {
	/// Returns a string representing the duration as latex commands with symbols using the LaTeX package `{siunitx}`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::{LatexSym, TexOptions};
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 1 ).to_latex_sym( &TexOptions::new() ),
	///     r"\qty{1}{\second}"
	/// );
	/// assert_eq!(
	///     NormTimeDelta::new_seconds( 10 ).to_latex_sym( &TexOptions::new() ),
	///     r"\qty{10}{\second}"
	/// );
	/// ```
	#[cfg( feature = "tex" )]
	fn to_latex_sym( &self, _options: &TexOptions ) -> String {
		format!( r"\qty{{{}}}{{\second}}", self.secs )
	}
}


#[cfg( feature = "serde" )]
mod normtime_serde {
	use super::NormTimeDelta;

	use std::fmt;

	impl serde::Serialize for NormTimeDelta {
		fn serialize<S>( &self, serializer: S ) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			serializer.serialize_i64( self.secs )
		}
	}

	struct NormTimeDeltaVisitor;

	impl<'de> serde::de::Visitor<'de> for NormTimeDeltaVisitor {
		type Value = NormTimeDelta;

		fn expecting( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
			formatter.write_str( "an integer between -2^63 and 2^63" )
		}

		fn visit_i8<E>( self, value: i8 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value as i64 ) )
		}

		fn visit_i16<E>( self, value: i16 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value as i64 ) )
		}

		fn visit_i32<E>( self, value: i32 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value as i64 ) )
		}

		fn visit_i64<E>( self, value: i64 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value ) )
		}

		fn visit_u8<E>( self, value: u8 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value as i64 ) )
		}

		fn visit_u16<E>( self, value: u16 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value as i64 ) )
		}

		fn visit_u32<E>( self, value: u32 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok( NormTimeDelta::new_seconds( value as i64 ) )
		}

		fn visit_u64<E>( self, value: u64 ) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			if value <= i64::MAX as u64 {
				return Ok( NormTimeDelta::new_seconds( value as i64 ) );
			}

			Err( E::custom( format!( "u64 out of range: {}", value ) ) )
		}
	}

	impl<'de> serde::Deserialize<'de> for NormTimeDelta {
		fn deserialize<D>( deserializer: D ) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			deserializer.deserialize_i64( NormTimeDeltaVisitor )
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
	fn test_last_digit() {
		assert_eq!( last_digit( 5 ), 5 );
		assert_eq!( last_digit( 11 ), 1 );
		assert_eq!( last_digit( 23 ), 3 );
		assert_eq!( last_digit( 123 ), 3 );
		assert_eq!( last_digit( 1234 ), 4 );
		assert_eq!( last_digit( 12345 ), 5 );
	}

	#[test]
	fn create_normtimedelta() {
		// Unix-time zero.
		assert_eq!( NormTimeDelta::new_seconds( 0 ), NormTimeDelta::ZERO );
		assert_eq!( NormTimeDelta::new_days( 1 ).seconds(), DUR_NORMDAY );
	}

	#[test]
	fn calculate_normtimedelta() {
		assert_eq!(
			NormTimeDelta::new_seconds( 0 ) + NormTimeDelta::new_seconds( 10 ),
			NormTimeDelta::new_seconds( 10 )
		);

		assert_eq!(
			NormTimeDelta::new_seconds( 10 ) + NormTimeDelta::new_seconds( 10 ),
			NormTimeDelta::new_seconds( 20 )
		);

		assert_eq!(
			NormTimeDelta::new_seconds( 10 ) - NormTimeDelta::new_seconds( 5 ),
			NormTimeDelta::new_seconds( 5 )
		);
	}

	#[test]
	fn calculate_sum_over_iterator() {
		let items = [
			NormTimeDelta::new_seconds( 10 ),
			NormTimeDelta::new_seconds( 11 ),
			NormTimeDelta::new_seconds( 12 ),
		];

		assert_eq!( items.iter().sum::<NormTimeDelta>(), NormTimeDelta::new_seconds( 33 ) );
		assert_eq!( items.into_iter().sum::<NormTimeDelta>(), NormTimeDelta::new_seconds( 33 ) );
	}

	#[test]
	fn tesxt_from_chrono_timedelta() {
		assert_eq!( NormTimeDelta::from( TimeDelta::seconds( 10 ) ), NormTimeDelta::new_seconds( 10 ) );
		assert_eq!( NormTimeDelta::from( TimeDelta::hours( 10 ) ), NormTimeDelta::new_hours( 10 ) );
		assert_eq!( NormTimeDelta::from( TimeDelta::new( 10, 1111 ).unwrap() ), NormTimeDelta::new( 10, 1111 ).unwrap() );
	}

	#[test]
	fn time_delta_display() {
		assert_eq!( NormTimeDelta::new_seconds( 1 ).to_string(), "1 second" );
		assert_eq!( NormTimeDelta::new_seconds( 10 ).to_string(), "10 seconds" );
	}

	#[test]
	#[cfg( feature = "serde" )]
	fn test_serialize_deserilaize() {
		assert_tokens(
			&NormTimeDelta::new_seconds( 10 ),
			&[ Token::I64( 10 ), ]
		);

		assert_tokens(
			&NormTimeDelta::new_years( 10 ),
			&[ Token::I64( 10 * DUR_NORMYEAR ), ]
		);
	}
}
