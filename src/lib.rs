//! Tracking and calculating time as Normtime.
//!
//! Normtime aspires to track time in a "metric" way. The goal is to have rough similarities to the standard time, but not being equal. a "Normday" is not equal to a standard day with 24 hours.
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




//=============================================================================
// Crates

mod time;
pub use crate::time::NormTime;
mod duration;
pub use crate::duration::NormTimeDelta;




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
