use crate::rpcs::prelude::*;

use lib_core::model::dsource::{DSource, DSourceBmc, DSourceFilter, DSourceForUpdate};

pub fn router_builder() -> RouterBuilder {
	router_builder!(
		// -- Generated CRUD
		dsource_list,
		dsource_update,
		dsource_delete,
	)
}

gen_rpc_crud_fns!(
	Bmc: DSourceBmc,
	Entity: DSource,
	ForUpdate: DSourceForUpdate,
	ForList: DSource,
	Filter: DSourceFilter,
	Suffix: dsource
);
