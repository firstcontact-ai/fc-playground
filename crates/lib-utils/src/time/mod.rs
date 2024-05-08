// region:    --- Modules
mod error;
#[cfg(all(feature = "for-ts", feature = "for-sql"))]
mod unix_time;

// -- Flatten
pub use self::error::{Error, Result};
pub use time::format_description::well_known::Rfc3339;

#[cfg(all(feature = "for-ts", feature = "for-sql"))]
pub use unix_time::*;

use time::{Duration, OffsetDateTime};

// endregion: --- Modules

// region:    --- Utc

pub fn now_utc() -> OffsetDateTime {
	OffsetDateTime::now_utc()
}

pub fn now_utc_fmt() -> String {
	format_time(OffsetDateTime::now_utc())
}

pub fn format_time(time: OffsetDateTime) -> String {
	time.format(&Rfc3339).unwrap() // TODO: need to check if safe.
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
	let new_time = now_utc() + Duration::seconds_f64(sec);
	format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
	OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

// endregion: --- Utc
