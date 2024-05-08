use crate::Result;
use lib_ais::agent_defs;
use lib_core::model::agent::AgentBmc;
use lib_core::model::ModelManager;

pub async fn init_agents(mm: &ModelManager) -> Result<()> {
	let agent = AgentBmc::first(mm, None, None).await?;

	if agent.is_none() {
		let agent_c = agent_defs::agent_one_c();

		let _agent_id = AgentBmc::create(mm, agent_c).await?;
	}

	Ok(())
}
