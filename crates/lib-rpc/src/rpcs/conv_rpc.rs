use crate::rpcs::prelude::*;

use lib_core::model::conv::{Conv, ConvBmc, ConvFilter, ConvForCreate, ConvForUpdate, ConvMsg};
use lib_core::model::msg::Msg;
use lib_core::model::stack_step::{StackStep, StackStepLite};
use lib_core::model::Id;
use rpc_router::RpcParams;
use serde::{Deserialize, Serialize};

pub fn router_builder() -> RouterBuilder {
	router_builder!(
		// -- Generated CRUD
		conv_create,
		conv_list,
		conv_update,
		conv_delete,
		// -- Customs
		conv_list_msgs,
		conv_list_steps,
		conv_get_step,
		conv_clear_all,
	)
}

gen_rpc_crud_fns!(
	Bmc: ConvBmc,
	Entity: Conv,
	ForCreate: ConvForCreate,
	ForUpdate: ConvForUpdate,
	ForList: Conv,
	Filter: ConvFilter,
	Suffix: conv
);

async fn conv_list_msgs(mm: ModelManager, conv_id: ParamsIded) -> Result<DataRpcResult<Vec<Msg>>> {
	let conv_id: Id = conv_id.id.into();
	let conv_msgs = ConvBmc::list_msgs(&mm, conv_id).await?;
	Ok(conv_msgs.into())
}

async fn conv_clear_all(mm: ModelManager, conv_id: ParamsIded) -> Result<DataRpcResult<()>> {
	let conv_id: Id = conv_id.id.into();
	ConvBmc::clear_all(&mm, conv_id).await?;
	Ok(().into())
}

#[derive(Serialize, Deserialize, RpcParams)]
pub struct ParamsListSteps {
	conv_id: i64,
	orig_msg_id: i64,
}

async fn conv_list_steps(mm: ModelManager, params: ParamsListSteps) -> Result<DataRpcResult<Vec<StackStepLite>>> {
	let steps = ConvBmc::list_steps(&mm, params.conv_id.into(), params.orig_msg_id.into()).await?;
	Ok(steps.into())
}

#[derive(Serialize, Deserialize, RpcParams)]
pub struct ParamsGetStep {
	conv_id: i64,
	step_id: i64,
}

async fn conv_get_step(mm: ModelManager, params: ParamsGetStep) -> Result<DataRpcResult<StackStep>> {
	let step = ConvBmc::get_step(&mm, params.conv_id.into(), params.step_id.into()).await?;
	Ok(step.into())
}
