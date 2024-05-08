// region:    --- Modules

// -- Privates
mod support;

// -- Flatten
pub use support::*;

// -- Public - main-db RPCs
pub mod agent_rpc;
pub mod conv_rpc;
pub mod drive_rpc;
pub mod dsource_rpc;
pub mod space_rpc;
// -- Public - AI RPCs
pub mod ai_rpc;

use rpc_router::RouterBuilder;

// endregion: --- Modules

pub fn all_rpc_router_buider() -> RouterBuilder {
	RouterBuilder::default()
		// -- Main Db
		.extend(agent_rpc::router_builder())
		.extend(space_rpc::router_builder())
		.extend(drive_rpc::router_builder())
		.extend(dsource_rpc::router_builder())
		.extend(conv_rpc::router_builder())
		// -- AI
		.extend(ai_rpc::router_builder())
}
