# normtime

This Rust crate supports tracking time as "Normtime".

Normtime aspires to represent time in a “metric” way. The goal is to have rough similarities to the standard time, but not being equal. A “Normday” has a duration of exact 100'000 seconds (27 h, 46 min, 40 s), and is therefore a couple of hours longer than a standard day with 24 hours, but at least roughly equivalent.

Normtime has the following attributes:
* The second is identical to the second as defined by the International System of Units (SI).
* The higher order units are simple multiples of a second:
	* 1 normday   := 100 ks (ca. 1 standard day: 27 h 46 min 40 s)
	* 1 normweek  :=   1 Ms (ca. 12 standard days: 11 d 13 h 46 min 40 s)
	* 1 normmonth :=   3 Ms (ca. 35 standard days: 34 d 17 h 20 min)
	* 1 normyear  :=  30 Ms (ca. 1 standard year: 347 d 5 h 20 min)
* The 0-point of the normtime is offset from Unix time by 3'089'836'800 seconds (2068-01-01T00:00:00 standard time)


## Usage

To use this crate your `Cargo.toml` could look like this:

```toml
[dependencies]
normtime = "0.1.0"
```

## Example

```
use normtime::NormTime;

let ntime = NormTime::from_ymd_opt( 123, 4, 5 ).unwrap().and_hms( 6, 7, 8 );
assert_eq!( ntime.to_string(), "0123-04-05N06:07:08" );
```

`NormTime` can be converted to `chrono::NaiveDateTime` and the other way around:
```
use chrono::{NaiveDateTime, NaiveDate};
use normtime::NormTime;

let ntime = NormTime::from_ymd_opt( 123, 4, 5 ).unwrap().and_hms( 6, 7, 8 );
let ndt = NaiveDate::from_ymd_opt( 2185, 4, 30 ).unwrap().and_hms_opt( 6, 20, 28 ).unwrap();

assert_eq!( NaiveDateTime::from( ntime ), ndt );
assert_eq!( NormTime::from( ndt ), ntime );
```

Durations between `NormTime`s are measured using `NormTimeDelta`.
```
use normtime::{NormTime, NormTimeDelta};

let ntime_start = NormTime::from_ymd_opt( 123, 4, 5 ).unwrap().and_hms( 6, 7, 8 );
let ntime_stop = NormTime::from_ymd_opt( 123, 4, 6 ).unwrap().and_hms( 6, 7, 8 );
assert_eq!( ntime_stop - ntime_start, NormTimeDelta::new_seconds( 100_000 ) );
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
