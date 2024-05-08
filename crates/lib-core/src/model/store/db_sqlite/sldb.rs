use crate::event::ModelEvent;
use crate::model::store::{Error, Result};
use crate::model::ModelPublisher;
use modql::FromSqliteRow;
use rusqlite::types::FromSql;
use rusqlite::{Connection, OptionalExtension, Params};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SlDb {
	can_write: bool,
	publisher: ModelPublisher,
	conn: Arc<Mutex<Connection>>,
}

// Private constructor
impl SlDb {
	pub(in crate::model) fn from_connection(conn: Connection, publisher: ModelPublisher, can_write: bool) -> Self {
		Self {
			can_write,
			publisher,
			conn: Arc::new(Mutex::new(conn)),
		}
	}
}

// Public exec SQL apis
impl SlDb {
	/// Executed a parameterized sql with its params, and return the number of rows affected
	/// returns: number of rows affected
	pub fn exec(&self, sql: &str, params: impl Params) -> Result<usize> {
		self.assert_can_write(sql)?;

		let conn_g = self.conn.lock()?;

		let row_affected = conn_g.execute(sql, params)?;
		Ok(row_affected)
	}

	/// Perform a sql exec and return the first row and first value as num
	/// e.g., `db.exec_as_num("select count(*) from person", [] )`
	pub fn exec_returning_num(&self, sql: &str, params: impl Params) -> Result<i64> {
		self.assert_can_write(sql)?;

		let conn_g = self.conn.lock()?;

		let mut stmt = conn_g.prepare(sql)?;
		// Note: Assume the first column is the id to be returned.
		let id = stmt.query_row(params, |r| r.get::<_, i64>(0))?;

		Ok(id)
	}

	/// Perform a seql exect and returns the first value of the first row and
	/// cast it to the type T
	/// ```
	/// let sql = r#"
	/// SELECT conv.cfile_id
	/// FROM conv
	/// JOIN space ON conv.space_id = space.id
	/// WHERE space.id = ?1
	/// ORDER BY conv.last_open DESC
	/// LIMIT 1;
	/// "#;
	/// let cfile_id: Option<Id> = mm.main_db().exec_as(sql, (space_id,))?;
	/// ```
	pub fn exec_returning_as<T: FromSql>(&self, sql: &str, params: impl Params) -> Result<T> {
		self.assert_can_write(sql)?;

		let conn_g = self.conn.lock()?;

		let mut stmt = conn_g.prepare(sql)?;
		// Note: Assume the first column is the id to be returned.
		let res = stmt.query_row(params, |r| r.get::<_, T>(0))?;

		Ok(res)
	}

	pub fn exec_returning_as_optional<T: FromSql>(&self, sql: &str, params: impl Params) -> Result<Option<T>> {
		self.assert_can_write(sql)?;

		let conn_g = self.conn.lock()?;

		let mut stmt = conn_g.prepare(sql)?;
		// Note: Assume the first column is the id to be returned.
		let res = stmt.query_row(params, |r| r.get::<_, T>(0)).optional()?;

		Ok(res)
	}

	pub fn fetch_first<P, T>(&self, sql: &str, params: P) -> Result<Option<T>>
	where
		P: Params,
		T: FromSqliteRow,
	{
		self.assert_can_write(sql)?;

		let all: Vec<T> = self.fetch_all(sql, params)?;

		Ok(all.into_iter().next())
	}

	pub fn fetch_all<P, T>(&self, sql: &str, params: P) -> Result<Vec<T>>
	where
		P: Params,
		T: FromSqliteRow,
	{
		self.assert_can_write(sql)?;

		let conn_g = self.conn.lock()?;
		let mut stmt = conn_g.prepare(sql)?;
		let iter = stmt.query_and_then(params, |r| T::from_sqlite_row(r))?;
		let mut res = Vec::new();
		for item in iter {
			res.push(item?)
		}
		Ok(res)
	}
}

// Public publish event
impl SlDb {
	pub async fn publish(&self, evt: ModelEvent) {
		self.publisher.publish(evt).await
	}
}

// private api
impl SlDb {
	fn assert_can_write(&self, sql: &str) -> Result<()> {
		if !self.can_write {
			if let Some(first_el) = sql.trim().split(' ').next().map(|s| s.to_lowercase()) {
				if first_el != "select" {
					return Err(Error::InvalidSqlOnNoWriteSlDb { sql: sql.to_string() });
				}
			}
		}
		Ok(())
	}
}

// This is just some test support additional SlDb
// #[cfg(test)]
#[cfg(any(test, feature = "for-test"))]
mod test_support {
	use super::*;

	impl SlDb {
		pub fn print_table(&self, table: &str) -> Result<()> {
			let conn_g = self.conn.lock()?;
			pretty_sqlite::print_table(&conn_g, table).map_err(Error::custom)?;

			Ok(())
		}

		pub fn print_select(&self, sql: &str) -> Result<()> {
			let conn_g = self.conn.lock()?;
			pretty_sqlite::print_select(&conn_g, sql, []).map_err(Error::custom)?;

			Ok(())
		}
	}
}
