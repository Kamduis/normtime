//! The measurement of duration or time delta in Normtime.




//=============================================================================
// Crates


use std::iter::Sum;
use std::ops::{Add, Sub};

use crate::{DUR_NORMDAY, DUR_NORMYEAR};




//=============================================================================
// Duration

/// Time duration with second precision.
///
/// `NormTimeDelta` differs from e.g. `chrono::TimeDelta`, that it uses normdays, normweeks etc. that have a different duration than standard days etc. The duration of a second is identical, though.
#[derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug )]
pub struct NormTimeDelta( pub(super) i64 );

impl NormTimeDelta {
	/// Creates a new `NormTimeDelta` that has a duration of zero seconds.
	pub const ZERO: Self = Self( 0 );

	/// Creates a new `NormTimeDelta` that has a duration of `seconds`.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_seconds( 0 ), NormTimeDelta::ZERO );
	/// ```
	pub fn new_seconds( seconds: i64 ) -> Self {
		Self( seconds )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `days` normdays.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	///
	/// assert_eq!( NormTimeDelta::new_days( 1 ), NormTimeDelta::new_seconds( 100_000 ) );
	/// ```
	pub fn new_days( days: i64 ) -> Self {
		Self( days * DUR_NORMDAY )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `years` normyears.
	///
	/// # Example
	///
	/// ```
	/// use normtime::NormTimeDelta;
	/// assert_eq!( NormTimeDelta::new_years( 1 ), NormTimeDelta::new_seconds( 30_000_000 ) );
	/// ```
	pub fn new_years( years: i64 ) -> Self {
		Self( years * DUR_NORMYEAR )
	}

	/// Returns `true` if `self` has a duration of 0 seconds.
	pub fn is_zero( &self ) -> bool {
		self.0 == 0
	}

	/// Returns the duration of `self` in seconds.
	pub fn seconds( &self ) -> i64 {
		self.0
	}
}

/// Adding two `NormTimeDelta` together returns the sum of the duration of both.
///
/// # Example
///
/// ```
/// use normtime::NormTimeDelta;
///
/// assert_eq!(
/// 	NormTimeDelta::new_seconds( 1 ) + NormTimeDelta::new_seconds( 10 ),
/// 	NormTimeDelta::new_seconds( 11 )
/// );
/// ```
impl Add for NormTimeDelta {
	type Output = Self;

	fn add( self, other: NormTimeDelta ) -> Self::Output {
		Self( self.0 + other.0 )
	}
}


/// Subtracting two `NormTimeDelta`.
///
/// # Example
///
/// ```
/// use normtime::NormTimeDelta;
///
/// assert_eq!(
/// 	NormTimeDelta::new_seconds( 1 ) - NormTimeDelta::new_seconds( 10 ),
/// 	NormTimeDelta::new_seconds( -9 )
/// );
/// ```
impl Sub for NormTimeDelta {
	type Output = Self;

	fn sub( self, other: Self ) -> Self::Output {
		Self( self.0 - other.0 )
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


#[cfg( feature = "serde" )]
mod normtime_serde {
	use super::NormTimeDelta;

	use std::fmt;

	use serde;

	impl serde::Serialize for NormTimeDelta {
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

			serializer.serialize_i64( ( *self ).0 )
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
			} else {
				return Err( E::custom( format!( "u64 out of range: {}", value ) ) );
			}
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

	use serde_test::{Token, assert_tokens};

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
