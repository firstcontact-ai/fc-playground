use crate::_test_support::Result;
use crate::chain::{AgentNode, AgentRef};
use lib_core::model::agent::{Agent, AgentBmc, AgentForCreate, OutFormat};
use lib_core::model::ModelManager;
use lib_utils::s;

/// Seed/Create in the db a list of Echo Aagent for the given [(name, instruction)] array
pub async fn seed_mock_echo_agents(mm: &ModelManager, data: &[(&str, &str)]) -> Result<Vec<Agent>> {
	let mut agents = Vec::new();
	for (name, inst) in data {
		let agent_c = mock_echo_agent_c(*name, *inst);
		let id = AgentBmc::create(mm, agent_c).await?;
		let agent = AgentBmc::get(mm, id).await?;
		agents.push(agent);
	}
	Ok(agents)
}

/// Return a Echo AgentForCreate
pub fn mock_echo_agent_c(name: impl Into<String>, inst: impl Into<String>) -> AgentForCreate {
	AgentForCreate {
		name: name.into(),
		space_default: None,
		model: Some(s!("fc-mock-echo-inst")),
		inst: Some(inst.into()),
		prompt_tmpl: None,
		desc: Some(s!("Just echo the instruction")),
		out_format: Some(OutFormat::Text),
		kind: None,
		provider: None,
	}
}

pub fn mock_name_from_agent_node(agent_node: &AgentNode) -> String {
	match &agent_node.agent {
		AgentRef::Name(name) => s!(name),
		AgentRef::Uid(uid) => s!(uid),
		AgentRef::Same => s!("same"),
		AgentRef::Id(id) => s!(id),
	}
}
