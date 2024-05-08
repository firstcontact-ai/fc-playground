use crate::model::ditem_dsource::DItemDSourceBmc;
use crate::model::dsource::DSourceIden;
use crate::model::support::prelude::*;
use modql::field::{HasFields, HasSeaFields};
use modql::filter::OpValString;
use sea_query::{Condition, Expr, Iden, IntoColumnRef, JoinType, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use simple_fs::SPath;
use std::str::FromStr;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, SeaFieldValue, FromSqliteValue, Serialize, Deserialize)]
pub enum DItemKind {
	Md,
	Pdf,
	Unknown,
}

impl FromStr for DItemKind {
	type Err = DItemError;

	fn from_str(s: &str) -> DItemResult<Self> {
		let spath = SPath::from_path(s).map_err(|ex| DItemError::CantParseDItemKind { cause: ex.to_string() })?;
		let ext = spath.extension().map(|s| s.to_lowercase());

		match ext.as_deref() {
			Some("md") => Ok(DItemKind::Md),
			Some("pdf") => Ok(DItemKind::Md),
			_ => Ok(DItemKind::Unknown),
		}
	}
}

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[serde_as]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct DItem {
	pub id: Id,
	pub uid: String,
	pub kind: DItemKind,

	pub folder_path: String,
	pub file_path: String,
	pub file_mtime: Option<UnixTimeUs>,
	pub file_size: Option<i64>,
	pub file_ext: Option<String>,

	pub proc_time: Option<UnixTimeUs>,
	pub dfile_id: Option<Id>,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

/// Note: Does not need implement d/serialize because it is use by a worker only.
#[derive(Fields)]
pub struct DItemForCreate {
	pub file_path: String,
	pub file_mtime: UnixTimeUs,
	pub file_size: i64,
	pub file_ext: String,

	pub folder_path: String,
}

/// Internal type to create the DItem.
/// Note: Computed only by this model. Should not be visible outside of this module
#[derive(Fields)]
struct DItemForCreateRec {
	pub kind: DItemKind,
	pub dfile_id: Option<i64>,

	pub folder_path: String,

	pub file_path: String,
	pub file_mtime: UnixTimeUs,
	pub file_size: i64,
	pub file_ext: String,
}

/// Same as the ForCreate, no need to implement d/serialize.
#[derive(Fields, Default)]
pub struct DItemForUpdate {
	pub dfile_id: Option<i64>,
	pub file_path: Option<String>,
	pub file_mtime: Option<UnixTimeUs>,
	pub file_size: Option<i64>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct DItemFilter {
	pub id: Option<OpValsInt64>,

	pub folder_path: Option<OpValsString>,

	pub file_path: Option<OpValsString>,
	pub file_ext: Option<OpValsString>,

	pub proc_time: Option<OpValsInt64>,
	pub dfile_id: Option<OpValsInt64>,
}

// Note: Camel case for variant names need to match snake cases of db.
#[derive(Iden)]
pub enum DItemIden {
	DfileId,
	FileMtime,
	ProcTime,
	FileExt,

	// -- For FKs
	DitemId,
}
// endregion: --- Types

// region:    --- DItemBmc

pub struct DItemBmc;

impl DbBmc for DItemBmc {
	const TABLE: &'static str = "ditem";
}

gen_mm_crud_fns!(
	Bmc: DItemBmc,
	ForGet: DItem,
	ForUpdate: DItemForUpdate,
	ForList: DItem,
	Filter: DItemFilter,
);

impl DItemBmc {
	/// Custom create that will create a DItemForCreateRec
	pub async fn create(mm: &ModelManager, ditem_c: DItemForCreate) -> Result<Id> {
		let kind: DItemKind = ditem_c.file_path.parse()?;

		let ditem_rec_c = DItemForCreateRec {
			kind,
			dfile_id: None, // TODO: this might be part of the ditem_c in the future.
			file_path: ditem_c.file_path,
			file_mtime: ditem_c.file_mtime,
			file_size: ditem_c.file_size,
			folder_path: ditem_c.folder_path,
			file_ext: ditem_c.file_ext,
		};

		let id = base::create::<Self, _>(mm.main_db(), ditem_rec_c).await?;

		Ok(id)
	}

	/// List the DItems base on the file_path base.
	pub async fn list_for_base_path(mm: &ModelManager, base_path: &str) -> Result<Vec<DItem>> {
		let filter = DItemFilter {
			file_path: Some(OpValString::StartsWith(base_path.into()).into()),
			..Default::default()
		};
		let ditems = DItemBmc::list(mm, Some(vec![filter]), None).await?;

		Ok(ditems)
	}

	/// List the DItems that
	/// - `ext` extension lowercase withou the `.` (.e.g., `md`)
	pub async fn list_ditems_without_dfile_for_dsource(mm: &ModelManager, dsource_id: Id) -> Result<Vec<DItem>> {
		// -- Build query
		// NOTE: Here we are writting it the sea-query builder way.
		//       There are pros/cons to use sea-query for those relatively complex sql.
		//       The code will experiment with both until a clear winner arise.
		// expr `dsource_id = dsource_id`
		let expr_dsource_id = Expr::col((DItemDSourceBmc::table_iden(), DSourceIden::DsourceId)).eq(dsource_id);

		// condition
		// `dfile_is IS NULL AND dsource_id = dsource_id`
		let cond = Condition::all()
			.add(Expr::col(DItemIden::DfileId).is_null())
			.add(expr_dsource_id);

		let columns = DItem::sea_column_refs_with_rel(DItemBmc::table_iden());

		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.join(
				JoinType::LeftJoin,
				DItemDSourceBmc::table_ref(),
				Expr::col((DItemDSourceBmc::table_iden(), DItemIden::DitemId).into_column_ref())
					.equals((DItemBmc::table_iden(), CommonIden::Id).into_column_ref()),
			)
			.columns(columns)
			.cond_where(cond);

		// -- Exec query
		let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
		let entities: Vec<DItem> = mm.main_db().fetch_all(&sql, &*values.as_params())?;

		Ok(entities)
	}

	// pub async fn list_d
	pub async fn list_ditems_to_proc_for_dsource(mm: &ModelManager, dsource_id: Id) -> Result<Vec<DItem>> {
		let columns: Vec<String> = DItem::field_names().iter().map(|n| format!(r#""ditem"."{n}""#)).collect();
		let columns = columns.join(",");
		let sql = format!(
			r#"
SELECT   {columns}
FROM     ditem 
JOIN     ditem_dsource ON ditem.id =  ditem_dsource.ditem_id 
WHERE    ditem_dsource.dsource_id  =  :dsource_id
AND      ditem.dfile_id            IS NOT NULL 
AND (
	       ditem.proc_time           IS NULL
OR 
         ditem.proc_time           <  ditem.file_mtime
);		
"#
		);

		let entities: Vec<DItem> = mm.main_db().fetch_all(&sql, &[(":dsource_id", &*dsource_id)])?;

		Ok(entities)
	}
}

// endregion: --- DItemBmc

// region:    --- DItemError

pub type DItemResult<T> = core::result::Result<T, DItemError>;

#[derive(Debug, Serialize)]
pub enum DItemError {
	CantParseDItemKind { cause: String },
}

impl core::fmt::Display for DItemError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for DItemError {}

// endregion: --- DItemError

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;

	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_dsource_rref = "src/";
		let fx_file_path = format!("{fx_dsource_rref}/lib.rs");
		let fx_file_ext = "rs".to_string();

		// -- Exec
		let ditem_c = DItemForCreate {
			file_path: fx_file_path.clone(),
			file_mtime: (-1).into(), // just for test, negative, meaning not computed.
			file_size: -1,
			file_ext: fx_file_ext,

			folder_path: format!("{fx_dsource_rref}/"),
		};
		let ditem_id = DItemBmc::create(&mm, ditem_c).await?;

		// -- Check
		let ditem = DItemBmc::get(&mm, ditem_id).await?;
		assert_eq!(ditem.id, 1.into()); // in memory db, we can assume 1 for the first
		assert_eq!(ditem.file_path, fx_file_path);

		Ok(())
	}
}

// endregion: --- Tests
