use crate::rpcs::prelude::*;
use lib_core::model::agent::Agent;
use lib_core::model::conv::Conv;
use lib_core::model::drive::Drive;
use lib_core::model::space::{Space, SpaceBmc, SpaceFilter, SpaceForCreate, SpaceForUpdate};

pub fn router_builder() -> RouterBuilder {
	router_builder!(
		// Same as RpcRouter::new().add...
		space_get,
		space_create,
		space_list,
		space_update,
		space_delete,
		// -- custom
		space_get_latest,
		space_get_default_drive,
		space_get_latest_conv,
		space_seek_agent
	)
}

gen_rpc_crud_fns!(
	Bmc: SpaceBmc,
	Entity: Space,
	ForCreate: SpaceForCreate,
	ForUpdate: SpaceForUpdate,
	ForList: Space,
	Filter: SpaceFilter,
	Suffix: space
);

async fn space_seek_agent(mm: ModelManager, spaced: ParamsIded) -> Result<DataRpcResult<Option<Agent>>> {
	let agent = SpaceBmc::seek_agent(&mm, spaced.id.into()).await?;
	Ok(agent.into())
}

async fn space_get_latest(mm: ModelManager) -> Result<DataRpcResult<Space>> {
	let space = SpaceBmc::get_latest(&mm).await?;
	Ok(space.into())
}

async fn space_get_latest_conv(mm: ModelManager, spaced: ParamsIded) -> Result<DataRpcResult<Conv>> {
	let conv = SpaceBmc::get_latest_conv(&mm, spaced.id.into()).await?;
	Ok(conv.into())
}

async fn space_get_default_drive(mm: ModelManager, spaced: ParamsIded) -> Result<DataRpcResult<Drive>> {
	let drive = SpaceBmc::get_default_drive(&mm, spaced.id.into()).await?;

	Ok(drive.into())
}
