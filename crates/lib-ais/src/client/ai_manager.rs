// region:    --- Modules

use crate::client::{AiClient, FcClient, OllamaClient};
use crate::{ClientKind, OpenaiClient, Result};
use lib_utils::x_vec::XStringVec;

// endregion: --- Modules

#[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Default, Clone)]
pub struct AiManager {
	ollama_client: OllamaClient,
	openai_client: OpenaiClient,
	fc_client: FcClient,
}

/// Public methods
impl AiManager {
	pub async fn get_client_for_model(&self, model_name: &str) -> Result<Box<dyn AiClient + Send>> {
		let client_type = self.get_provider_for_model(model_name).await?;

		self.get_client(client_type)
	}

	pub async fn list_all_models(&self) -> Result<Vec<String>> {
		let mut models = Vec::new();

		models.extend(self.ollama_client.list_models().await?);
		models.extend(self.openai_client.list_models().await?);
		models.extend(self.fc_client.list_models().await?);

		Ok(models)
	}
}

/// Private methods
impl AiManager {
	fn get_client(&self, kind: ClientKind) -> Result<Box<dyn AiClient + Send>> {
		match kind {
			ClientKind::Ollama => Ok(Box::new(self.ollama_client.clone())),
			ClientKind::Openai => Ok(Box::new(self.openai_client.clone())),
			ClientKind::Fc => Ok(Box::new(self.fc_client.clone())),
		}
	}

	/// Returns the provider type for a model name
	/// FIXME: Right now, just check the mock model, and then, differ to Ollama
	///        Will need to have full implementation, and do some caching.
	async fn get_provider_for_model(&self, model_name: &str) -> Result<ClientKind> {
		if self.fc_client.list_models().await?.x_contains(model_name) {
			Ok(ClientKind::Fc)
		}
		// FIXME: Right now fall back on Ollama
		else {
			Ok(ClientKind::Ollama)
		}
	}
}
