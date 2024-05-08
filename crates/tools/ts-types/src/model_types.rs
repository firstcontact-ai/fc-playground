#![allow(unused)] // Those are just types for generation.

use lib_core::model;

// NOTE: This is just the root placeholder type
//       to have one generation and avoid any possible duplication.
//       There might be a better or more elegant way to do this.
//       This type will be removed from the final .ts file
#[allow(non_camel_case_types)]
#[derive(schemars::JsonSchema)]
pub struct Types_PLACEHOLDER {
	// -- Agent
	agent: model::agent::Agent,
	agent_l: model::agent::AgentLite,
	agent_c: model::agent::AgentForCreate,
	agent_u: model::agent::AgentForUpdate,
	// -- Space
	space: model::space::Space,
	space_c: model::space::SpaceForCreate,
	space_u: model::space::SpaceForUpdate,
	// -- Conv
	conv: model::conv::Conv,
	// -- Drive
	drive: model::drive::Drive,
	drive_c: model::drive::DriveForCreate,
	drive_u: model::drive::DriveForUpdate,
	// -- DSource
	dsource: model::dsource::DSource,
	dsource_c: model::dsource::DSourceForCreate,
	dsource_u: model::dsource::DSourceForUpdate,
}
