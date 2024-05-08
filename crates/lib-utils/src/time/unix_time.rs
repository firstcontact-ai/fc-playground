use super::{Error, Result};
use crate::derive_simple_data_type;
use std::time::{SystemTime, UNIX_EPOCH};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

derive_simple_data_type! {
	/// Unix Epoch time in Micosecond precision
	pub struct UnixTimeUs(i64);
}

/// Returns the UnitTimeUs for now.
pub fn now() -> UnixTimeUs {
	UnixTimeUs(now_unix_time_us())
}

/// Returns the current unix/epoch time in microseconds.
///
/// Note 1: In the unlikely event of a failure, it will return the start of the EPOCH.
///
/// Note 2: The maximum UTC epoch date that can be stored in i64 with microseconds precision
///         would be approximately `292277-01-09 ... UTC`.
///         Thus, for all practical purposes, it is sufficiently distant to be of no concern.
pub fn now_unix_time_us() -> i64 {
	let now = SystemTime::now();
	let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::new(0, 0));

	since_the_epoch.as_micros().min(i64::MAX as u128) as i64
}

/// Returns the RFC3339 format of an epoch in microseconds.
///
/// Note: In the unlikely event of a failure, it will return the
///       start of the EPOCH formatted as RFC3339 "1970-01-01T00:00:00Z".
///       Other strategies may also be valid.
pub fn unix_us_to_rfc3339(epoch_us: i64) -> String {
	let epoch_us: i128 = epoch_us as i128 * 1000;

	// In the unlikely event of a failure, return the start of the EPOCH.
	let time = OffsetDateTime::from_unix_timestamp_nanos(epoch_us).unwrap_or(OffsetDateTime::UNIX_EPOCH);

	// This should probably never fail.
	time.format(&Rfc3339).unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

/// Returns the epoch in microseconds for a date-time string formatted in RFC3339.
///
/// Note: Given the higher probability of error with this function,
///       it will return a `Result<T>`.
///       Other strategies may also be valid.
pub fn rfc3339_to_unix_us(time_rfc3339: &str) -> Result<i64> {
	let time =
		OffsetDateTime::parse(time_rfc3339, &Rfc3339).map_err(|_| Error::FailToDateParse(time_rfc3339.to_string()))?;
	let time_ms = time.unix_timestamp_nanos() / 1000;
	let time_ms = time_ms.min(i64::MAX as i128) as i64;
	Ok(time_ms)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use serde_json::Value;

	#[test]
	fn test_serde_epoch_time() -> Result<()> {
		// -- Setup & Fixtures
		let fx_ep = UnixTimeUs(123);

		// -- Exec
		let value: Value = serde_json::to_value(fx_ep)?;
		let ep: UnixTimeUs = serde_json::from_value(value.clone())?;

		// -- Check
		assert_eq!(ep, fx_ep);
		assert_eq!(value, Value::Number(123.into()));

		Ok(())
	}

	#[test]
	fn test_time_epoch() -> Result<()> {
		// -- Setup & Fixtures
		let fx_epoch_us = now_unix_time_us();

		// -- Exec
		let epoch_rfc3339 = unix_us_to_rfc3339(fx_epoch_us);
		let epoch_us = rfc3339_to_unix_us(&epoch_rfc3339)?;

		// -- Check
		assert_eq!(fx_epoch_us, epoch_us);

		Ok(())
	}
}
