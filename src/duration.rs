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
	/// Creates a new `NormTimeDelta` that is zero.
	const ZERO: Self = Self( 0 );

	/// Creates a new `NormTimeDelta` that has a duration of `seconds`.
	pub fn new_seconds( seconds: i64 ) -> Self {
		Self( seconds )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `days` normdays.
	pub fn new_days( days: i64 ) -> Self {
		Self( days * DUR_NORMDAY )
	}

	/// Creates a new `NormTimeDelta` that has a duration of `years` normyears.
	pub fn new_years( years: i64 ) -> Self {
		Self( years * DUR_NORMYEAR )
	}

	/// Returns `true` if `self` is 0.
	pub fn is_zero( &self ) -> bool {
		self.0 == 0
	}

	/// Returns the length of `self` in seconds.
	pub fn seconds( &self ) -> i64 {
		self.0
	}
}

impl Add for NormTimeDelta {
	type Output = Self;

	fn add( self, other: NormTimeDelta ) -> Self::Output {
		Self( self.0 + other.0 )
	}
}

impl Sub for NormTimeDelta {
	type Output = Self;

	fn sub( self, other: Self ) -> Self::Output {
		Self( self.0 - other.0 )
	}
}

impl Sum for NormTimeDelta {
	fn sum<I>( iter: I ) -> Self
	where
		I: Iterator<Item = Self>
	{
		iter.fold( Self( 0 ), |acc, x| acc + x )
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

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
	fn normtimedelta_sum() {
		let accumulated: NormTimeDelta = [
			NormTimeDelta::new_seconds( 10 ),
			NormTimeDelta::new_seconds( 10 ),
			NormTimeDelta::new_seconds( 10 ),
		].into_iter().sum();

		assert_eq!( accumulated, NormTimeDelta::new_seconds( 30 ) );
	}
}
