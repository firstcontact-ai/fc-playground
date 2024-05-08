//! Lower level generic based CRUD functions to be used by BMC directly or via declartive macros.
//!
//! Note:
//! - Those function take the &SlDb (and not the ModelManager) to be able to be reused by the
//!   main_db BMC with ModelManager or the dfile_db with a SlDb directly.

use crate::model::base::prep_fields::{prep_fields_for_create, prep_fields_for_update};
use crate::model::base::{CommonIden, DbBmc, Id, LIST_LIMIT_DEFAULT, LIST_LIMIT_MAX};
use crate::model::store::db_sqlite::SlDb;
use crate::model::{Error, Result};
use modql::field::{HasSeaFields, SeaFields};
use modql::filter::{FilterGroups, ListOptions};
use modql::FromSqliteRow;
use sea_query::{Condition, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

pub async fn create<MC, E>(db: &SlDb, data: E) -> Result<Id>
where
	MC: DbBmc,
	E: HasSeaFields,
{
	// -- Extract fields (name / sea-query value expression)
	let fields = data.not_none_sea_fields();

	// -- Exec with create_from_fields
	create_with_fields::<MC>(db, fields).await
}

/// FIXME: Somehow, if the async fn create does an .await (on this one for example)
///        we get a compile arror in the rpc router dyn
///
/// Lower create from the fields themselves.
/// Note: This function will call the `prep_fields_for_create` on the fields.
pub async fn create_with_fields<MC>(db: &SlDb, mut fields: SeaFields) -> Result<Id>
where
	MC: DbBmc,
{
	// -- Prep Fields
	prep_fields_for_create::<MC>(&mut fields);

	// -- Build query
	let (columns, sea_values) = fields.for_sea_insert();
	let mut query = Query::insert();
	query.into_table(MC::table_ref()).columns(columns).values(sea_values)?;
	query.returning_col(CommonIden::Id);

	// -- Exec query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let id = db.exec_returning_num(&sql, &*values.as_params())?;

	// -- Publish Model Event
	MC::publish_create_event(db, id.into()).await;

	Ok(id.into())
}

pub async fn get<MC, E>(db: &SlDb, id: Id) -> Result<E>
where
	MC: DbBmc,
	E: FromSqliteRow + Unpin + Send,
	E: HasSeaFields,
{
	// -- Build query
	let mut query = Query::select();
	query
		.from(MC::table_ref())
		.columns(E::sea_column_refs())
		.and_where(Expr::col(CommonIden::Id).eq(id));

	// -- Exec query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let entity = db
		.fetch_first(&sql, &*values.as_params())?
		.ok_or(Error::EntityNotFound { entity: MC::TABLE, id })?;

	Ok(entity)
}

pub async fn get_by_uid<MC, E>(db: &SlDb, uid: &str) -> Result<E>
where
	MC: DbBmc,
	E: FromSqliteRow + Unpin + Send,
	E: HasSeaFields,
{
	// -- Build query
	let mut query = Query::select();
	query
		.from(MC::table_ref())
		.columns(E::sea_column_refs())
		.and_where(Expr::col(CommonIden::Uid).eq(uid));

	// -- Exec query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let entity = db.fetch_first(&sql, &*values.as_params())?.ok_or(Error::EntityByUidNotFound {
		entity: MC::TABLE,
		uid: uid.to_string(),
	})?;

	Ok(entity)
}

pub async fn first<MC, E, F>(db: &SlDb, filter: Option<F>, list_options: Option<ListOptions>) -> Result<Option<E>>
where
	MC: DbBmc,
	F: Into<FilterGroups>,
	E: FromSqliteRow + Unpin + Send,
	E: HasSeaFields,
{
	let list_options = match list_options {
		Some(mut list_options) => {
			// Reset the offset/limit
			list_options.offset = None;
			list_options.limit = Some(1);

			// Don't change order_bys if not empty,
			// otherwise, set it to id (creation asc order)
			list_options.order_bys = list_options.order_bys.or_else(|| Some("id".into()));

			list_options
		}
		None => ListOptions {
			limit: Some(1),
			offset: None,
			order_bys: Some("id".into()), // default id asc
		},
	};

	list::<MC, E, F>(db, filter, Some(list_options))
		.await
		.map(|item| item.into_iter().next())
}

pub async fn list<MC, E, F>(db: &SlDb, filter: Option<F>, list_options: Option<ListOptions>) -> Result<Vec<E>>
where
	MC: DbBmc,
	F: Into<FilterGroups>,
	E: FromSqliteRow + Unpin + Send,
	E: HasSeaFields,
{
	// -- Build the query
	let mut query = Query::select();
	query.from(MC::table_ref()).columns(E::sea_column_refs());

	// condition from filter
	if let Some(filter) = filter {
		let filters: FilterGroups = filter.into();
		let cond: Condition = filters.try_into()?;
		query.cond_where(cond);
	}
	// list options
	let list_options = compute_list_options(list_options)?;
	list_options.apply_to_sea_query(&mut query);

	// -- Execute the query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

	let entities = db.fetch_all(&sql, &*values.as_params())?;

	Ok(entities)
}

pub async fn update<MC, E>(db: &SlDb, id: Id, data: E) -> Result<()>
where
	MC: DbBmc,
	E: HasSeaFields,
{
	// -- Extract Fields
	let fields = data.not_none_sea_fields();

	// -- Exec
	update_with_fields::<MC>(db, id, fields).await
}

// FIXME: See the one create. Same issue
pub async fn update_with_fields<MC>(db: &SlDb, id: Id, mut fields: SeaFields) -> Result<()>
where
	MC: DbBmc,
{
	// -- Prep fields
	prep_fields_for_update::<MC>(&mut fields);

	// -- Build query
	let fields = fields.for_sea_update();
	let mut query = Query::update();
	query
		.table(MC::table_ref())
		.values(fields)
		.and_where(Expr::col(CommonIden::Id).eq(id));

	// -- Execute query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let row_affected = db.exec(&sql, &*values.as_params())?;

	// -- Check result
	if row_affected == 0 {
		Err(Error::EntityNotFound { entity: MC::TABLE, id })
	} else {
		// -- Publish Model Event
		MC::publish_update_event(db, id).await;
		Ok(())
	}
}

pub async fn delete<MC>(db: &SlDb, id: Id) -> Result<()>
where
	MC: DbBmc,
{
	// -- Build query
	let mut query = Query::delete();
	query.from_table(MC::table_ref()).and_where(Expr::col(CommonIden::Id).eq(id));

	// -- Execute query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let row_affected = db.exec(&sql, &*values.as_params())?;

	// -- Check result
	if row_affected == 0 {
		Err(Error::EntityNotFound { entity: MC::TABLE, id })
	} else {
		// -- Publish Model Event
		MC::publish_delete_event(db, id).await;
		Ok(())
	}
}

pub fn compute_list_options(list_options: Option<ListOptions>) -> Result<ListOptions> {
	if let Some(mut list_options) = list_options {
		// Validate the limit.
		if let Some(limit) = list_options.limit {
			if limit > LIST_LIMIT_MAX {
				return Err(Error::ListLimitOverMax {
					max: LIST_LIMIT_MAX,
					actual: limit,
				});
			}
		}
		// Set the default limit if no limit
		else {
			list_options.limit = Some(LIST_LIMIT_DEFAULT);
		}
		Ok(list_options)
	}
	// When None, return default
	else {
		Ok(ListOptions {
			limit: Some(LIST_LIMIT_DEFAULT),
			offset: None,
			order_bys: Some("id".into()),
		})
	}
}
