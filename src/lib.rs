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
//! 	* 1 normday   := 100 ks (ca. 1 standard day)
//! 	* 1 normweek  :=   1 Ms (ca. 12 standard days)
//! 	* 1 normmonth :=   3 Ms (ca. 35 standard days)
//! 	* 1 normyear  :=  30 Ms (ca. 1 standard year, ca. 347 standard days)
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
//! * **serde** Enables `serde` support.




//=============================================================================
// Crates

mod time;
pub use crate::time::NormTime;
mod duration;
pub use crate::duration::{NormTimeDelta, Unit};




//=============================================================================
// Constants


/// The offset between unix time and norm time in seconds.
const NORMTIME_OFFSET: i64 = 3_092_601_600;

/// The duration of a normyear in seconds.
const DUR_NORMYEAR: i64 = 30_000_000;

/// The duration of a normmonth in seconds.
const DUR_NORMMONTH: i64 = 3_000_000;

/// The duration of a normweek in seconds.
const DUR_NORMWEEK: i64 = 1_000_000;

/// The duration of a normday in seconds.
const DUR_NORMDAY: i64 = 100_000;

/// The duration of an hour in seconds.
const DUR_HOUR: i64 = 3600;

/// The duration of a minute in seconds.
const DUR_MINUTE: i64 = 60;
