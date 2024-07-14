//! Tracking and calculating time as Normtime.
//!
//! Normtime aspires to track time in a "metric" way. The goal is to have rough similarities to the standard time, but not being equal. a "Normday" is not equal to a standard day with 24 hours.
//!
//! # NormTime
//!
//! Normtime has the following basics:
//!
//! * The second is identical to the second as defined by the International System of Units (SI).
//! * The higher order units are simple multiples of a second:
//!     * 1 normday   := 100 ks (ca. 1 standard day)
//!     * 1 normweek  :=   1 Ms (ca. 12 standard days)
//!     * 1 normmonth :=   3 Ms (ca. 35 standard days)
//!     * 1 normyear  :=  30 Ms (ca. 1 standard year, ca. 347 standard days)
//! * The 0-point of the normtime is offset from Unix time by 3'089'836'800 seconds (2068-01-01T00:00:00 standard time)
//!
//! It is easy to crate a `NormTime`:
//! ```
//! use normtime::NormTime;
//!
//! let ntime = NormTime::from_ymd_opt( 123, 4, 5 ).unwrap().and_hms( 6, 7, 8 );
//! assert_eq!( ntime.to_string(), "0123-04-05N06:07:08" );
//! ```
//!
//! `NormTime` can be converted to `chrono::NaiveDateTime` and the other way around:
//! ```
//! use chrono::{NaiveDateTime, NaiveDate};
//! use normtime::NormTime;
//!
//! let ntime = NormTime::from_ymd_opt( 123, 4, 5 ).unwrap().and_hms( 6, 7, 8 );
//! let ndt = NaiveDate::from_ymd_opt( 2185, 4, 30 ).unwrap().and_hms_opt( 6, 20, 28 ).unwrap();
//!
//! assert_eq!( NaiveDateTime::from( ntime ), ndt );
//! assert_eq!( NormTime::from( ndt ), ntime );
//! ```
//!
//! # NormTimeDelta
//!
//! Durations between `NormTime`s are measured using `NormTimeDelta`.
//! ```
//! use normtime::{NormTime, NormTimeDelta};
//!
//! let ntime_start = NormTime::from_ymd_opt( 123, 4, 5 ).unwrap().and_hms( 6, 7, 8 );
//! let ntime_stop = NormTime::from_ymd_opt( 123, 4, 6 ).unwrap().and_hms( 6, 7, 8 );
//! assert_eq!( ntime_stop - ntime_start, NormTimeDelta::new_seconds( 100_000 ) );
//! ```
//!
//! # Optional Features
//! * **i18n** Enables internationalization support using `fluent_templates`.
//! * **serde** Enables `serde` support.
//! * **tex** Enables LaTeX support.




//=============================================================================
// Crates


#[cfg( all( feature = "i18n", feature = "tex" ) )]
use unic_langid::LanguageIdentifier;

mod time;
pub use crate::time::NormTime;
mod duration;
pub use crate::duration::{NormTimeDelta, Unit};




//=============================================================================
// Traits


/// Providing conversion into LaTeX code.
///
/// This Trait is only available, if the **`tex`** feature has been enabled.
#[cfg( feature = "tex" )]
pub trait Latex {
	/// Converts the entity into a LaTeX-string.
	fn to_latex( &self, options: &TexOptions ) -> String;

	/// Converts the entity into a LaTeX-string translating it into the language provided by `locale`.
	#[cfg( feature = "i18n" )]
	fn to_latex_locale( &self, locale: &LanguageIdentifier, options: &TexOptions ) -> String;

	/// Converts the entity into a LaTeX-string displaying symbols instead of written units.
	fn to_latex_sym( &self, options: &TexOptions ) -> String;
}




//=============================================================================
// Constants


/// The offset between unix time and norm time in seconds.
const NORMTIME_OFFSET: i64 = 3_092_601_600;

/// The duration of a normyear in seconds.
pub const DUR_NORMYEAR: i64 = 30_000_000;

/// The duration of a normmonth in seconds.
const DUR_NORMMONTH: i64 = 3_000_000;

/// The duration of a normweek in seconds.
const DUR_NORMWEEK: i64 = 1_000_000;

/// The duration of a normday in seconds.
const DUR_NORMDAY: i64 = 100_000;

/// The duration of a earth year in seconds.
pub const DUR_TERRAYEAR: i64 = 31_557_600;

/// The duration of an hour in seconds.
const DUR_HOUR: i64 = 3600;

/// The duration of a minute in seconds.
const DUR_MINUTE: i64 = 60;




//=============================================================================
// Structs


/// Representing options to LaTeX commands provided by the `Latex` trait.
///
/// **Note:** This struct is only available, if the **`tex`** feature has been enabled.
///
/// TODO: This is currently not being used, but will provide the possibility to fine-tune the TeX output. It is added to keep the function signatures stable.
#[cfg( feature = "tex" )]
#[derive( PartialEq, Default, Debug )]
pub struct TexOptions {}

#[cfg( feature = "tex" )]
impl TexOptions {
	// Create a new `Options` without an option active. Is identical to `none()`.
	pub fn new() -> Self {
		Self::default()
	}

	// Create a new `Options` without an option active.
	pub fn none() -> Self {
		Self::default()
	}
}




//=============================================================================
// Internationalization


#[cfg( feature = "i18n" )]
fluent_templates::static_loader! {
	static LOCALES = {
		// The directory of localisations and fluent resources.
		locales: "./locales",

		// The language to falback on if something is not present.
		fallback_language: "en-US",
	};
}
