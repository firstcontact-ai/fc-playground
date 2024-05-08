use crate::model::base::CommonIden;
use crate::model::ditem::DItemIden;
use crate::model::support::prelude::*;
use lib_utils::time::UnixTimeUs;
use sea_query::Expr;
use sea_query::Query;
use sea_query::SqliteQueryBuilder;
use sea_query_rusqlite::RusqliteBinder;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[serde_as]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct DItemDSource {
	pub id: Id,

	pub ditem_id: Id,
	pub dsource_id: Id,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

/// Note: Does not need implement d/serialize because it is use by a worker only.
#[derive(Fields)]
pub struct DItemDSourceForCreate {
	pub ditem_id: Id,
	pub dsource_id: Id,
}

/// Note: Does not need implement d/serialize because it is use by a worker only.
///       Also, can be empty, as for now, just to update mitem (which is added by base::update on update)
#[derive(Fields)]
pub struct DItemDSourceForUpdate {}

#[derive(FilterNodes, Default, Deserialize)]
pub struct DItemDSourceFilter {
	pub id: Option<OpValsInt64>,

	pub ditem_id: Option<OpValsInt64>,
	pub dsource_id: Option<OpValsInt64>,

	pub ctime: Option<OpValsInt64>,
	pub mtime: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- DItemDSourceBmc

pub struct DItemDSourceBmc;

impl DbBmc for DItemDSourceBmc {
	const TABLE: &'static str = "ditem_dsource";

	fn has_uid() -> bool {
		false
	}
}

gen_mm_crud_fns!(
	Bmc: DItemDSourceBmc,
	ForGet: DItemDSource,
	ForCreate: DItemDSourceForCreate,
	ForList: DItemDSource,
	Filter: DItemDSourceFilter,
);

/// This is a many-to-many table, so the `pks` is the `ditem_id, dsource_id`
impl DItemDSourceBmc {
	pub async fn get_id_from_pks(mm: &ModelManager, ditem_id: Id, dsource_id: Id) -> Result<Id> {
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.column(CommonIden::Id)
			.and_where(Expr::col(CommonIden::DsourceId).eq(dsource_id))
			.and_where(Expr::col(DItemIden::DitemId).eq(ditem_id));

		// -- Exec query
		let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
		let id = mm
			.main_db()
			.exec_returning_num(&sql, &*values.as_params())
			.map_err(|_| format!("DItemDSource not found for ditem_id: {ditem_id}, dsource_id: {dsource_id}"))?;

		Ok(id.into())
	}

	/// Update the `mtime` of this entity for its `pks` (i.e. `ditem_id, dsource_id`)
	pub async fn touch_from_pks(mm: &ModelManager, ditem_id: Id, dsource_id: Id) -> Result<Id> {
		let ditem_dsource_id = Self::get_id_from_pks(mm, ditem_id, dsource_id).await?;

		base::update::<Self, _>(mm.main_db(), ditem_dsource_id, DItemDSourceForUpdate {}).await?;

		Ok(ditem_dsource_id)
	}
}

// endregion: --- DItemDSourceBmc
