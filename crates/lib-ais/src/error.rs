use derive_more::From;
use lib_core::model::Id;
use lib_utils::hbs;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
	MutexPoison,

	// -- Chain
	ChainFailParse(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
	AgentChainHasNodeNode {
		agent_id: Id,
	},
	FailSerializeStack(#[serde_as(as = "DisplayFromStr")] serde_json::Error),

	// -- Runner
	CantRunStepStackEmpty {
		step_id: Id,
	},
	FailToHbsRenderPrompt(#[serde_as(as = "DisplayFromStr")] hbs::Error),

	// -- Stack
	StackFailParse(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
	StackStepNotFound(Id),
	PrevStepHasNoOuput(Id),
	PrevStepHasNoStack(Id),
	StepMsgHasNoContent(Id),

	// -- AiClient
	AiModelNotImplemented(String),

	// -- Model related
	SpaceHasNoAgent {
		space_id: Id,
	},
	AgentNotFoundForName(String),
	AgentHasNoModel {
		agent_id: Id,
		agent_name: String,
	},

	// -- App Libs
	#[from]
	Model(lib_core::model::Error),

	// -- Externals
	#[from]
	Ollama(#[serde_as(as = "DisplayFromStr")] ollama_rs::error::OllamaError),
	OllamaCustom(String),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
