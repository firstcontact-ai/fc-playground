use rpc_router::{IntoParams, RpcParams};
use serde::de::DeserializeOwned;
use serde::Deserialize;

// region:    --- RpcParams

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Deserialize, RpcParams)]
pub struct RunUserPromptParams {
	pub conv_id: i64,
	pub user_prompt: String,
}

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Deserialize)]
pub struct UserPrompt {
	pub prompt: String,
}

#[derive(Deserialize)]
pub struct SpacedAiParams<D> {
	pub space_id: i64,
	pub data: D,
}
impl<D> IntoParams for SpacedAiParams<D> where D: DeserializeOwned + Send {}

// endregion: --- RpcParams
