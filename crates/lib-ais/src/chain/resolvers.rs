use crate::chain::{AgentNode, AgentRef};
use crate::{Error, Result};
use lib_core::model::agent::{Agent, AgentBmc};
use lib_core::model::{Id, ModelManager};

pub async fn resolve_agent(mm: &ModelManager, base_agent_id: Id, chain_node: &AgentNode) -> Result<Agent> {
	let agent = match &chain_node.agent {
		// Take the base agent_id from argment
		AgentRef::Same => AgentBmc::get(mm, base_agent_id).await?,

		// Get uuid
		AgentRef::Uid(uid) => AgentBmc::get_by_uid(mm, uid).await?,

		// Get by id
		AgentRef::Id(id) => AgentBmc::get(mm, id.into()).await?,

		// Get by name
		AgentRef::Name(name) => AgentBmc::first_by_name(mm, name)
			.await?
			.ok_or_else(|| Error::AgentNotFoundForName(name.to_string()))?,
	};

	Ok(agent)
}
