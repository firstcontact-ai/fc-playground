use crate::Result;
use lib_core::model::agent::AgentBmc;
use lib_core::model::space::{SpaceBmc, SpaceForCreate};
use lib_core::model::ModelManager;
use lib_utils::s;

/// Check Initialize Spaces
pub async fn init_spaces(mm: &ModelManager) -> Result<()> {
	let first_space = SpaceBmc::first(mm, None, None).await?;

	let space_default_agent = AgentBmc::seek_space_default_agent(mm).await?;
	let agent_id = space_default_agent.map(|a| a.id);

	if first_space.is_none() {
		let space_c = SpaceForCreate {
			name: s!("Space One"),
			agent_id,
		};
		let _ = SpaceBmc::create(mm, space_c).await?;
	}

	Ok(())
}
