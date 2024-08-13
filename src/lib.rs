// Replace crate links with internal links when creating documentation with `cargo`.
//! [`NormTime`]: crate::NormTime
//! [`NormTimeDelta`]: crate::NormTimeDelta
//! [`DateTime`]: chrono::DateTime
//! [`NaiveDateTime`]: chrono::NaiveDateTime
//! [`fluent_templates`]: fluent_templates
//! [`serde`]: serde
// File links are not supported by rustdoc.
//! [LICENSE-APACHE]: https://github.com/Kamduis/normtime/blob/master/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/Kamduis/normtime/blob/master/LICENSE-MIT
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!( "../README.md" )]




//=============================================================================
// Crates


#[cfg( any( feature = "i18n", feature = "tex" ) )] use std::fmt;

#[cfg( feature = "i18n" )] use unic_langid::LanguageIdentifier;

mod time;
pub use crate::time::NormTime;
mod duration;
pub use crate::duration::{NormTimeDelta, Unit};




//=============================================================================
// Traits


/// Providing a localized `.to_string()`: `.to_string_locale()`.
///
/// This Trait is only available, if the **`i18n`** feature has been enabled.
#[cfg( feature = "i18n" )]
pub trait DisplayLocale: fmt::Display {
	/// Returns the localized string representation of `self`.
	///
	/// The standard implementation ignores `locale` and returns the same string as `.to_string()`.
	#[allow( unused_variables )]
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		self.to_string()
	}
}


/// Providing conversion into LaTeX code.
///
/// This Trait is only available, if the **`tex`** feature has been enabled.
#[cfg( feature = "tex" )]
pub trait Latex: fmt::Display {
	/// Converts the entity into a LaTeX-string.
	///
	/// The standard implementation ignores `options` and returns the same as `.to_string()`.
	#[allow( unused_variables )]
	fn to_latex( &self, options: &TexOptions ) -> String {
		self.to_string()
	}
}


/// Providing a localized `.to_latex()`: `.to_latex_locale()`.
///
/// This Trait is only available, if the both, the **`i18n`** and the **`tex`** features have been enabled.
#[cfg( all( feature = "i18n", feature = "tex" ) )]
pub trait LatexLocale: DisplayLocale + Latex {
	/// Returns the localized LaTeX string representation of `self`.
	///
	/// The standard implementation ignores `options` and returns the same string as `.to_string_locale()`.
	#[allow( unused_variables )]
	fn to_latex_locale( &self, locale: &LanguageIdentifier, options: &TexOptions ) -> String {
		self.to_string_locale( locale )
	}
}


/// Providing conversion into LaTeX code to print symbols instead of text. This is mostly implemented to print out time symbols like `\normyear` or `\second` (using the LaTeX package `{siunitx}` instead of words.
///
/// This Trait is only available, if the **`tex`** feature has been enabled.
#[cfg( feature = "tex" )]
pub trait LatexSym: Latex {
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
