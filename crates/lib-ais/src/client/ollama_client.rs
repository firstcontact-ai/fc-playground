use super::AiClient;
use crate::types::{GenResChunk, GenResStream};
use crate::{Error, GenReq, GenRes, Result};
use async_trait::async_trait;
use futures::StreamExt;
use lib_core::model::agent::OutFormat;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationResponse;
use ollama_rs::generation::parameters::FormatType;
use ollama_rs::Ollama;
use std::sync::Arc;
use tracing::debug;

#[derive(Default, Clone)]
pub struct OllamaClient {
	client: Arc<Ollama>,
}

#[async_trait]
impl AiClient for OllamaClient {
	async fn list_models(&self) -> Result<Vec<String>> {
		let ollama = &self.client;
		let models = ollama.list_local_models().await?;
		let models = models.into_iter().map(|m| m.name).collect();
		Ok(models)
	}

	async fn gen(&self, model: &str, req: GenReq) -> Result<GenRes> {
		let ollama = &self.client;

		let ola_req = req.into_ollama_req(model);
		debug!(
			"OllamaClient.gen model: {}, format: {:?}",
			ola_req.model_name, ola_req.format
		);
		let ola_res = ollama
			.generate(ola_req)
			.await
			.map_err(|oe| Error::OllamaCustom(oe.to_string()))?;
		debug!("OllamaClient.gen DONE");

		Ok(ola_res.into())
	}

	async fn gen_stream(&self, model: &str, req: GenReq) -> Result<GenResStream> {
		let ollama = &self.client;

		let ola_req = req.into_ollama_req(model);
		let ollama_stream = ollama.generate_stream(ola_req).await.unwrap();

		let stream = ollama_stream.map(|chunk_result| {
			chunk_result
				.map(|chunk| {
					let res: Vec<GenResChunk> = chunk.into_iter().map(GenResChunk::from).collect();
					res
				})
				.map_err(Error::Ollama)
		});

		Ok(Box::pin(stream))
	}
}

// region:    --- Custom Intos

impl GenReq {
	fn into_ollama_req(self, model: &str) -> GenerationRequest {
		let mut ola_req = GenerationRequest::new(model.to_string(), self.prompt);
		ola_req.system = self.inst;
		ola_req.format = match self.out_format {
			Some(OutFormat::Json) => Some(FormatType::Json),
			_ => None,
		};

		ola_req
	}
}

// endregion: --- Custom Intos

// region:    --- Froms

impl From<GenerationResponse> for GenResChunk {
	fn from(val: GenerationResponse) -> Self {
		GenResChunk { response: val.response }
	}
}

impl From<GenerationResponse> for GenRes {
	fn from(value: GenerationResponse) -> Self {
		Self {
			response: value.response,
		}
	}
}

// endregion: --- Froms

// region:    --- Tests

#[cfg(test)]
mod tests {
	#![allow(unused)] // For early development.

	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_test_support::print_gen_stream;

	// NOTE: For now, we just turn on those test when debugging.
	//       They take time, so, need see see the right strategy later.
	//       They MUST work when ucommented

	// #[tokio::test]
	async fn test_gen() -> Result<()> {
		// -- Setup & Fixtures
		let ola_client = OllamaClient::default();
		let fx_model = "mixtral";
		let fx_msg = "why the sky is red. Be very concise.";

		// -- Exec
		println!("start");
		let res = ola_client.gen(fx_model, fx_msg.into()).await?;
		println!("res:\n{}", res.response);

		// -- Check

		Ok(())
	}

	// #[tokio::test]
	async fn test_gen_stream() -> Result<()> {
		// -- Setup & Fixtures
		let ola_client = OllamaClient::default();
		let fx_model = "mixtral";
		let fx_msg = "why the sky is red. Be very concise.";

		// -- Exec
		println!("start");
		let stream = ola_client.gen_stream(fx_model, fx_msg.into()).await?;
		print_gen_stream(stream).await;
		println!("done");

		// -- Check

		Ok(())
	}
}

// endregion: --- Tests
