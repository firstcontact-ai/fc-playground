// region:    --- Modules

mod db_config;
mod sldb;
mod sql_splitter;

// -- Flatten
pub use sldb::*;

use super::Result;
use crate::lfs::app_user_dir;
use crate::model::store::db_sqlite::sql_splitter::split_sql;
use crate::model::{DbType, ModelPublisher};
use db_config::get_db_config;
use rusqlite::Connection;
use std::path::Path;

use modql::ToSqliteValue;

// endregion: --- Modules

#[derive(ToSqliteValue)]
pub struct Dd(i64);

// region:    --- Db Constructors

pub fn init_db(db_type: &DbType, publisher: ModelPublisher, uid: Option<&str>) -> Result<SlDb> {
	let db_config = get_db_config(db_type);

	// resolve the dir file
	let dir = app_user_dir().join(db_config.dir);

	// resolve the name,
	let file_name = if let Some(uid) = uid {
		db_config.file_name.replace("{uid}", uid)
	} else {
		db_config.file_name.to_string()
	};

	// file_path
	let file_path = dir.join(file_name);

	// get the conn and create db if needed
	let conn = new_sqlite_conn(&file_path)?;
	create_schema_if_needed(&conn, db_config.table_check, db_config.schema)?;

	Ok(SlDb::from_connection(conn, publisher, true))
}

pub fn create_schema_if_needed(conn: &Connection, table_name_check: &str, schema: &str) -> Result<()> {
	// -- Test if the table "space" has been created
	// Note: For now, we will assume schema is up to date if space is created
	let test_schema_sql = "SELECT count(name) FROM sqlite_master WHERE type='table' AND name=:table";
	let count = conn.query_row(test_schema_sql, &[(":table", table_name_check)], |r| r.get::<_, i64>(0))?;
	if count == 1 {
		return Ok(());
	}

	// -- Create the schemas
	// (simple split on ';' for now)
	let sqls = split_sql(schema);
	for sql in sqls {
		let sql = sql.trim();
		if !sql.is_empty() {
			let _r = conn.execute(
				sql,
				(), // empty list of parameters.
			)?;
		}
	}

	Ok(())
}

// endregion: --- Db Constructors

// region:    --- SQLite Connection Constructor

// #[cfg(not(test))]
#[cfg(not(any(test, feature = "for-test")))]
pub fn new_sqlite_conn(db_file: &Path) -> Result<Connection> {
	use simple_fs::ensure_file_dir;

	// -- Create dir and validate file path
	ensure_file_dir(db_file)?;
	let db_file_str = db_file.to_str().ok_or("Invalid main_db_filePath")?;

	// -- Create the db with this file
	let conn = Connection::open(db_file_str)?;

	Ok(conn)
}

/// For test, we ignore the db_file and create a in memory conn.
// #[cfg(test)]
#[cfg(any(test, feature = "for-test"))]
pub fn new_sqlite_conn(_db_file: &Path) -> Result<Connection> {
	let conn = Connection::open_in_memory()?;

	Ok(conn)
}

// endregion: --- SQLite Connection Constructor
