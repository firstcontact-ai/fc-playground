use crate::model::support::prelude::*;
use tracing::debug;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[skip_serializing_none]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct DFile {
	pub id: Id,

	pub uid: String,

	pub main_dsource_id: Option<Id>,
}

#[derive(Default, Fields)]
pub struct DFileForCreate {
	pub main_dsource_id: Option<Id>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct DFileFilter {
	pub uid: Option<OpValsString>,

	pub main_dsource_id: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- DFileBmc

pub struct DFileBmc;

impl DbBmc for DFileBmc {
	const TABLE: &'static str = "dfile";
}

gen_mm_crud_fns!(
	Bmc: DFileBmc,
	ForGet: DFile,
	ForCreate: DFileForCreate,
	ForList: DFile,
	Filter: DFileFilter,
);

impl DFileBmc {
	pub async fn get_or_create_for_dsource(mm: &ModelManager, dsource_id: Id) -> Result<DFile> {
		// -- Try to get the dfile for this dsource
		let dfile = DFileBmc::first(
			mm,
			Some(vec![DFileFilter {
				main_dsource_id: Some((*dsource_id).into()),
				..Default::default()
			}]),
			None,
		)
		.await?;

		// -- If cannot find it, create a new one.
		let dfile = match dfile {
			Some(dfile) => dfile,
			None => {
				let id = DFileBmc::create(
					mm,
					DFileForCreate {
						main_dsource_id: Some(dsource_id),
					},
				)
				.await?;
				let dfile = DFileBmc::get(mm, id).await?;
				debug!("New dfile (ui: {}) created for dsource (id: {})", dfile.uid, dsource_id);
				dfile
			}
		};

		Ok(dfile)
	}

	/// Returns the Sqlite DB object for a give dfile.
	/// Notes:
	/// - It's based on the dfile.uid (Something like that in `[fc-app-base]/dfiles/dfile-[uid].db3`)
	/// - Will be created (with schema) if it does not exists
	/// - Multiple `DItem`s can point to the same `DFile` (data file).
	/// - Typically affinity of `DItem` to `DFile` is based o nthe `DSource` that first created the `DItem`
	/// - Therefore same `dfile_db` will contain multiple ditem content.
	pub async fn get_dfile_db(mm: &ModelManager, dfile: &DFile) -> Result<SlDb> {
		let db = mm.dfile_db(&dfile.uid).await?;
		Ok(db)
	}

	/// Returns the list of distinct DFiles for a given dsource.
	pub async fn list_dfiles_for_dsource(mm: &ModelManager, dsource_id: Id) -> Result<Vec<DFile>> {
		// -- Build query
		// Note: Here we are building the name directly as the select is more readable.
		let columns: Vec<String> = DFile::field_names().iter().map(|n| format!(r#""dfile"."{n}""#)).collect();
		let columns = columns.join(",");
		let sql = format!(
			r#"
SELECT DISTINCT dfile.id, {columns}
FROM   dfile 
JOIN   ditem ON dfile.id = ditem.dfile_id 
JOIN   ditem_dsource ON ditem.id = ditem_dsource.ditem_id 
WHERE  ditem_dsource.dsource_id = :dsource_id;
"#,
		);

		// -- Exec query
		let entities: Vec<DFile> = mm.main_db().fetch_all(&sql, &[(":dsource_id", &*dsource_id)])?;

		Ok(entities)
	}
}

// endregion: --- DFileBmc
