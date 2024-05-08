#![allow(unused)] // For early development.

use super::AiClient;
use crate::{GenReq, GenRes, GenResStream, Result};
use async_openai::config::OpenAIConfig;
use async_openai::Client;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub type OaClient = Client<OpenAIConfig>;

#[derive(Clone)]
pub struct OpenaiClient {
	conn: Arc<Mutex<OaClient>>,
}

impl Default for OpenaiClient {
	fn default() -> Self {
		Self {
			conn: Arc::new(Mutex::new(Client::new())),
		}
	}
}

#[async_trait]
impl AiClient for OpenaiClient {
	async fn list_models(&self) -> Result<Vec<String>> {
		Ok(vec![])
	}

	async fn gen(&self, model: &str, req: GenReq) -> Result<GenRes> {
		todo!()
	}

	async fn gen_stream(&self, model: &str, req: GenReq) -> Result<GenResStream> {
		todo!()
	}
}
