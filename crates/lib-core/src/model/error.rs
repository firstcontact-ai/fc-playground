use crate::event;
use crate::model::agent::AgentError;
use crate::model::ditem::DItemError;
use crate::model::dsource::DSourceError;
use crate::model::msg::MsgError;
use crate::model::space::SpaceError;
use crate::model::{store, Id};
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
	#[from]
	Custom(String),

	EntityNotFound {
		entity: &'static str,
		id: Id,
	},

	EntityByUidNotFound {
		entity: &'static str,
		uid: String,
	},

	SrEntityNotFound {
		table: &'static str,
		thid: String,
	},

	ListLimitOverMax {
		max: i64,
		actual: i64,
	},

	// -- Event
	#[from]
	Event(event::Error),

	// -- main_db  Entities
	#[from]
	DSource(DSourceError),
	#[from]
	DItem(DItemError),
	#[from]
	Agent(AgentError),
	#[from]
	Space(SpaceError),

	// -- cfile_db Entities
	#[from]
	Msg(MsgError),

	// -- Space
	NoSpaceFound,
	NoDefaultSpaceDriveFound,

	// -- Conv
	NoConvFoundForSpace {
		space_id: Id,
	},

	// -- SQLite
	SqliteIdMustBeNumber {
		actual: String,
	},

	// -- Store
	UniqueViolation {
		msg: Option<String>,
	},
	Store(store::Error),

	// -- Externals
	#[from]
	SimpleFs(#[serde_as(as = "DisplayFromStr")] simple_fs::Error),
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
	#[from]
	SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
	#[from]
	ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
	#[from]
	Modql(#[serde_as(as = "DisplayFromStr")] modql::Error),
}

// region:    --- Froms

/// Note: Here we upgrade the store Error::UniqueViolation as a model Error::UniqueViolation
///       to make it more convenient for downstream code.
impl From<store::Error> for Error {
	fn from(val: store::Error) -> Self {
		match val {
			store::Error::UniqueViolation { msg } => Error::UniqueViolation { msg },
			_ => Error::Store(val),
		}
	}
}

impl From<&str> for Error {
	fn from(val: &str) -> Self {
		Self::Custom(val.to_string())
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
