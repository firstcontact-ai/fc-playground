use crate::rpcs::prelude::*;

use lib_core::model::drive::{Drive, DriveBmc, DriveFilter, DriveForCreate, DriveForUpdate};
use lib_core::model::dsource::{DSource, DSourceBmc, DSourceForCreate};

pub fn router_builder() -> RouterBuilder {
	router_builder!(
		// -- Generated CRUD
		drive_create,
		drive_list,
		drive_update,
		drive_delete,
		// -- Customs
		drive_add_dsource,
	)
}

gen_rpc_crud_fns!(
	Bmc: DriveBmc,
	Entity: Drive,
	ForCreate: DriveForCreate,
	ForUpdate: DriveForUpdate,
	ForList: Drive,
	Filter: DriveFilter,
	Suffix: drive
);

async fn drive_add_dsource(
	mm: ModelManager,
	dsource_c: ParamsForCreate<DSourceForCreate>,
) -> Result<DataRpcResult<DSource>> {
	let dsource_id = DriveBmc::add_dsource(&mm, dsource_c.data).await?;
	let dsource = DSourceBmc::get(&mm, dsource_id).await?;
	Ok(dsource.into())
}
