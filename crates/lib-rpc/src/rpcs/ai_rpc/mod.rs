// region:    --- Modules

// -- Imports
use crate::rpcs::prelude::*;
use crate::RunUserPromptParams;
// use lib_ais::runner::legacy_ai_conv_run;
use lib_ais::AiManager;
use lib_core::model::conv::ConvBmc;

// endregion: --- Modules

pub fn router_builder() -> RouterBuilder {
	router_builder!(
		//
		ai_run_user_prompt,
		ai_list_models,
	)
}

async fn ai_run_user_prompt(mm: ModelManager, params: RunUserPromptParams) -> Result<DataRpcResult<()>> {
	let RunUserPromptParams { conv_id, user_prompt } = params;

	ConvBmc::add_conv_msg(&mm, conv_id.into(), user_prompt.into()).await?;

	Ok(().into())
}

/// List the list of agents
async fn ai_list_models(aim: AiManager) -> Result<DataRpcResult<Vec<String>>> {
	let models = aim.list_all_models().await?;
	// For now,
	// - Filter the "embed" models out.
	// - remove the `::...` (keep just the first part)
	let models: Vec<String> = models
		.into_iter()
		.filter(|m| !m.contains("embed"))
		.map(|m| m.split(':').next().unwrap_or_default().to_string())
		.collect();

	Ok(models.into())
}
