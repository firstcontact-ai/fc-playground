use crate::model::support::prelude::*;
use lib_utils::f;
use modql::field::HasFields;

// region:    --- Types

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct Part {
	pub id: Id,
	pub uid: String,

	pub ditem_ref_id: i64,

	pub is_title: bool,
	pub level: i64,
	pub line_num: i64,
	pub content: String,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct PartForCreate {
	pub ditem_ref_id: i64,

	pub is_title: bool,
	pub level: i64,
	pub group: i64,
	pub line_num: i64,
	pub content: String,
}

// endregion: --- Types

// region:    --- Bmc

pub struct PartBmc;

impl DbBmc for PartBmc {
	const TABLE: &'static str = "part";
}

generate_sldb_crud_fns!(
	Bmc: PartBmc,
	ForGet: Part,
	ForCreate: PartForCreate,
);

impl PartBmc {
	pub async fn content_search(db: &SlDb, search: &str) -> Result<Vec<Part>> {
		let columns: Vec<String> = Part::field_names().iter().map(|n| f!(r#""part"."{n}""#)).collect();
		let columns = columns.join(",");

		let sql = format!(
			r#"
SELECT {columns}
FROM part_fts 
JOIN part ON part_fts.rowid = part.id 
WHERE part_fts MATCH :search;
"#
		);

		let entities: Vec<Part> = db.fetch_all(&sql, &[(":search", &search)])?;

		Ok(entities)
	}
}

// endregion: --- Bmc
