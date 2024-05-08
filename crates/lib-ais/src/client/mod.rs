// region:    --- Modules

mod ai_manager;
mod fc;
mod ollama_client;
mod openai_client;

pub use ai_manager::*;
pub use fc::*;
pub use ollama_client::*;
pub use openai_client::*;

use crate::{GenReq, GenRes, GenResStream, Result};
use async_trait::async_trait;

// endregion: --- Modules

pub enum ClientKind {
	Ollama,
	Openai,
	Fc,
}

#[async_trait]
pub trait AiClient {
	async fn list_models(&self) -> Result<Vec<String>>;

	async fn gen(&self, model: &str, req: GenReq) -> Result<GenRes>;

	async fn gen_stream(&self, model: &str, req: GenReq) -> Result<GenResStream>;
}
