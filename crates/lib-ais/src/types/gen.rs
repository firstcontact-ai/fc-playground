use crate::Result;
use futures::Stream;
use lib_core::model::agent::OutFormat;
use lib_core::model::conv::ConvMsg;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

// region:    --- GenReq

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize)]
pub struct GenReq {
	pub prompt: String,
	pub inst: Option<String>,
	pub out_format: Option<OutFormat>,
}

impl From<&str> for GenReq {
	fn from(val: &str) -> Self {
		Self {
			prompt: val.to_string(),
			inst: None,
			out_format: None,
		}
	}
}

impl From<String> for GenReq {
	fn from(val: String) -> Self {
		Self {
			prompt: val,
			inst: None,
			out_format: None,
		}
	}
}

impl From<&String> for GenReq {
	fn from(val: &String) -> Self {
		Self {
			prompt: val.to_string(),
			inst: None,
			out_format: None,
		}
	}
}

// endregion: --- GenReq

// region:    --- GenRes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenRes {
	pub response: String,
}

impl GenRes {
	pub fn into_conv_msg(self) -> ConvMsg {
		ConvMsg { content: self.response }
	}
}

// endregion: --- GenRes

// region:    --- GenResStream

pub type GenResStream = Pin<Box<dyn Stream<Item = Result<GenResChunks>>>>;

pub type GenResChunks = Vec<GenResChunk>;

pub struct GenResChunk {
	pub response: String,
}

// endregion: --- GenResStream
