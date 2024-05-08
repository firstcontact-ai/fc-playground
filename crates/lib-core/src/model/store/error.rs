use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
	#[from]
	Custom(String),

	InvalidSqlOnNoWriteSlDb {
		sql: String,
	},

	UniqueViolation {
		msg: Option<String>,
	},

	// -- Std
	MutexPoison,

	// -- External
	Rusqlite(#[serde_as(as = "DisplayFromStr")] rusqlite::Error),
	#[from]
	SimpleFs(#[serde_as(as = "DisplayFromStr")] simple_fs::Error),
}

// region:    --- Custom

impl From<&str> for Error {
	fn from(val: &str) -> Self {
		Self::Custom(val.to_string())
	}
}

impl Error {
	pub fn custom(val: impl std::fmt::Display) -> Self {
		Self::Custom(val.to_string())
	}
}

// endregion: --- Custom

// region:    --- Froms

impl From<rusqlite::Error> for Error {
	fn from(val: rusqlite::Error) -> Self {
		match val {
			rusqlite::Error::SqliteFailure(ex, msg) => match &ex.code {
				rusqlite::ErrorCode::ConstraintViolation => {
					// for unique violation
					if ex.extended_code == 2067 {
						Error::UniqueViolation { msg }
					} else {
						Error::Rusqlite(rusqlite::Error::SqliteFailure(ex, msg))
					}
				}
				_ => Error::Rusqlite(rusqlite::Error::SqliteFailure(ex, msg)),
			},
			_ => Error::Rusqlite(val),
		}
	}
}

impl<T> From<std::sync::PoisonError<T>> for Error {
	fn from(_val: std::sync::PoisonError<T>) -> Self {
		Self::MutexPoison
	}
}

// endregion: --- Froms

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
