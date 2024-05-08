use crate::client::AiClient;
use crate::{Error, Result};
use crate::{GenReq, GenRes, GenResStream};
use async_trait::async_trait;
use lib_utils::s;

const FC_MODEL_MOCK_ECHO_INST: &str = "fc-mock-echo-inst";
const FC_MODEL_MOCK_ECHO_PROMPT: &str = "fc-mock-echo-prompt";

#[derive(Clone, Default)]
pub struct FcClient {}

#[async_trait]
impl AiClient for FcClient {
	async fn list_models(&self) -> Result<Vec<String>> {
		Ok(vec![s!(FC_MODEL_MOCK_ECHO_INST)])
	}

	async fn gen(&self, model: &str, req: GenReq) -> Result<GenRes> {
		match model {
			FC_MODEL_MOCK_ECHO_INST => Ok(GenRes {
				response: req.inst.unwrap_or_default().to_string(),
			}),
			FC_MODEL_MOCK_ECHO_PROMPT => Ok(GenRes { response: req.prompt }),
			_ => Err(Error::AiModelNotImplemented(model.to_string())),
		}
	}

	async fn gen_stream(&self, _model: &str, _req: GenReq) -> Result<GenResStream> {
		todo!("MockClient gen_stream not implemented")
	}
}
