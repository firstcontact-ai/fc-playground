pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use crate::_test_support::seed_mock_echo_agents;
use lib_core::_test_support::seed_space;
use lib_core::model::agent::{Agent, AgentBmc, AgentForUpdate};
use lib_core::model::cfile::CFileBmc;
use lib_core::model::conv::Conv;
use lib_core::model::space::SpaceBmc;
use lib_core::model::support::prelude::SlDb;
use lib_core::model::ModelManager;

// region:    --- Chain

const CHAIN: &str = r#"
{
	"nodes": [
		{
			"agent": "self",
			"name_input": "original_input"
		}, 
		{
			"branch": [
				{ 
					"cond": {
						"input": {
							"is_json": true,
							"json_matches": {
								"pointer": "/category",
								"value": 1
							}
						}
					},
					"nodes": [{
						"agent": { "name": "fc_tool_executors" }
					}, {
						"agent": { "name": "fc_tool_renderers" }
					}]			
				},
				{
					"cond": {
						"input": {
							"is_json": true,
							"json_matches": {
								"pointer": "/category",
								"value": 2
							}
						}
					},
					"nodes": [{
						"agent": {"name": "Generic Agent"}
					}]					
				}			
			]
		},
		{
			"agent": {"name": "Final Agent"}
		}		
	]
}
"#;

// endregion: --- Chain

pub async fn seed_all_for_test_runner(mm: &ModelManager) -> Result<(SlDb, Conv)> {
	// Seed agents and chain
	let fx_agents = seed_test_runner_agents(mm).await?;
	let first_agent = fx_agents.first().ok_or("Should have at least one agent")?;

	// Seed the space and conv
	let fx_space_id = seed_space(mm, "Space One").await?;
	SpaceBmc::set_agent(mm, fx_space_id, first_agent.id).await?;
	let conv = SpaceBmc::get_latest_conv(mm, fx_space_id).await?;
	let cfile_db = CFileBmc::getc_cfile_db_for_conv(mm, &conv).await?;

	Ok((cfile_db, conv))
}

async fn seed_test_runner_agents(mm: &ModelManager) -> Result<Vec<Agent>> {
	let agents = seed_mock_echo_agents(
		mm,
		&[
			(
				"Agent One",
				r#"
{ 
	"category": 1, 
	"actions": [
		{
		 "action": "list_files",
	   "topics": ["finance"]
		}
	]
}	
	"#,
			),
			("fc_tool_executors", "fc_tool_executors response"),
			("fc_tool_renderers", "fc_tool_renderers response"),
			("Generic Agent", "Generic Agent response"),
			("Final Agent", "Final Agent response"),
		],
	)
	.await?;

	// -- Add the chain
	let first_agent = agents.first().ok_or("Should have at least one agent")?;
	AgentBmc::update(
		mm,
		first_agent.id,
		AgentForUpdate {
			chain: Some(CHAIN.to_string()),
			..Default::default()
		},
	)
	.await?;

	Ok(agents)
}
