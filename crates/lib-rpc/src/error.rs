use derive_more::From;
use lib_core::model::Id;
use rpc_router::RpcHandlerError;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize, RpcHandlerError)]
pub enum Error {
	MissingCtx,

	// -- RPC Router
	RpcMethodUnknown(String),
	RpcIntoParamsMissing,

	SpaceHasNoAgent {
		space_id: Id,
	},
	AgentHasNoModel {
		agent_id: Id,
		agent_name: String,
	},

	// -- Modules
	#[from]
	Model(#[serde_as(as = "DisplayFromStr")] lib_core::model::Error),
	#[from]
	Ais(lib_ais::Error),

	// -- External Modules
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
