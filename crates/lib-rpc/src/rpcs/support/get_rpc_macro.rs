/// Create the base crud rpc functions following the common pattern.
/// - `create_...`
/// - `get_...`
///
/// NOTE: Make sure to import the Ctx, ModelManager, ... in the model that uses this macro.
#[macro_export]
macro_rules! gen_rpc_crud_fns {
	(
        Bmc: $bmc:ident,
        Entity: $entity:ty,
        $(ForCreate: $for_create:ty,)?
        $(ForUpdate: $for_update:ty,)?
        $(
				ForList: $for_list:ty,
				Filter: $filter:ty,
				)?
        Suffix: $suffix:ident
    ) => {
		paste! {
		$(
			pub async fn [<$suffix _create >](
				mm: ModelManager,
				params: ParamsForCreate<$for_create>,
			) -> Result<DataRpcResult<$entity>> {
				let ParamsForCreate { data } = params;
				let id = $bmc::create(&mm, data).await?;
				let entity = $bmc::get(&mm, id).await?;
				Ok(entity.into())
			}
		)?

		#[allow(unused)] // For early development.
		pub async fn [<$suffix _get>](
			mm: ModelManager,
			params: ParamsIded,
		) -> Result<DataRpcResult<$entity>> {
			let entity = $bmc::get(&mm, params.id.into()).await?;
			Ok(entity.into())
		}

		$(
			// Note: for now just add `s` after the suffix.
			pub async fn [<$suffix _list>](
				mm: ModelManager,
				params: ParamsList<$filter>,
			) -> Result<DataRpcResult<Vec<$for_list>>> {
				let entities = $bmc::list(&mm, params.filters, params.list_options).await?;
				Ok(entities.into())
			}
		)?

		$(
			pub async fn [<$suffix _update>](
				mm: ModelManager,
				params: ParamsForUpdate<$for_update>,
			) -> Result<DataRpcResult<$entity>> {
				let ParamsForUpdate { id, data } = params;
				let id = id.into();
				$bmc::update(&mm, id, data).await?;
				let entity = $bmc::get(&mm, id).await?;
				Ok(entity.into())
			}
		)?

			pub async fn [<$suffix _delete>](
				mm: ModelManager,
				params: ParamsIded,
			) -> Result<DataRpcResult<$entity>> {
				let ParamsIded { id } = params;
				let id = id.into();
				let entity = $bmc::get(&mm, id).await?;
				$bmc::delete(&mm, id).await?;
				Ok(entity.into())
			}

		}
	};
}
