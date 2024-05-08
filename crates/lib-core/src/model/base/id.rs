use crate::model::{Error, Result};
// use derive_more::{Deref, Display, From, Into};
use lib_utils::derive_simple_data_type;

// Simple wrapper for SQLite Ids
derive_simple_data_type! {
	pub struct Id(i64);
}

impl Id {
	pub fn as_i64(&self) -> i64 {
		self.0
	}
}

// from &i64
impl From<&i64> for Id {
	fn from(val: &i64) -> Id {
		Id(*val)
	}
}

impl TryFrom<String> for Id {
	type Error = Error;
	fn try_from(val: String) -> Result<Id> {
		let id = val.parse().map_err(|_| Error::SqliteIdMustBeNumber { actual: val })?;
		Ok(Id(id))
	}
}
