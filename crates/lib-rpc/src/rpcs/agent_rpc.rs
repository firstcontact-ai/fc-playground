use crate::rpcs::prelude::*;
use lib_core::model::agent::{Agent, AgentBmc, AgentFilter, AgentForCreate, AgentForUpdate, AgentLite};

pub fn router_builder() -> RouterBuilder {
	router_builder!(
		// -- Commons
		agent_get,
		agent_create,
		agent_list,
		agent_update,
		agent_delete,
	)
}

gen_rpc_crud_fns!(
	Bmc: AgentBmc,
	Entity: Agent,
	ForCreate: AgentForCreate,
	ForUpdate: AgentForUpdate,
	ForList: AgentLite,
	Filter: AgentFilter,
	Suffix: agent
);
