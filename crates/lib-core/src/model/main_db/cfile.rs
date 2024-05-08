use crate::model::conv::{Conv, ConvBmc, ConvForUpdate};
use crate::model::support::prelude::*;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[skip_serializing_none]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct CFile {
	pub id: Id,

	pub uid: String,
}

#[derive(Default, Fields)]
pub struct CFileForCreate {}

#[derive(FilterNodes, Default, Deserialize)]
pub struct CFileFilter {
	pub uid: Option<OpValsString>,

	pub main_dsource_id: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- CFileBmc

pub struct CFileBmc;

impl DbBmc for CFileBmc {
	const TABLE: &'static str = "cfile";
}

gen_mm_crud_fns!(
	Bmc: CFileBmc,
	ForGet: CFile,
	ForCreate: CFileForCreate,
	ForList: CFile,
	Filter: CFileFilter,
);

impl CFileBmc {
	/// Get (and create if needed) the db file for
	pub async fn getc_cfile_db_for_conv(mm: &ModelManager, conv: &Conv) -> Result<SlDb> {
		let cfile = Self::getc_for_conv(mm, conv).await?;
		let db = Self::getc_cfile_db(mm, &cfile).await?;
		Ok(db)
	}
}

// privates
impl CFileBmc {
	async fn getc_for_conv(mm: &ModelManager, conv: &Conv) -> Result<CFile> {
		let cfile_id = CFileBmc::getc_for_conv_inner(mm, conv).await?;
		let cfile = CFileBmc::get(mm, cfile_id).await?;
		Ok(cfile)
	}

	/// Get or create the Sqlite DB object for a give cfile by its uid
	/// Notes:
	/// - It's based on the dfile.uid (Something like that in `[fc-app-base]/cfiles/cfile-[uid].db3`)
	async fn getc_cfile_db(mm: &ModelManager, dfile: &CFile) -> Result<SlDb> {
		let db = mm.cfile_db(&dfile.uid).await?;
		Ok(db)
	}

	// Get or create the cfile row and returns the attached cfile_id for this conv_id
	async fn getc_for_conv_inner(mm: &ModelManager, conv: &Conv) -> Result<Id> {
		// -- Get from conv
		if let Some(cfile_id) = conv.cfile_id {
			return Ok(cfile_id);
		}

		// -- If no cfile_id, try to find a existing cfile candidate
		let cfile_id = Self::find_candidate_for_space_id(mm, conv.space_id).await?;

		// -- If no cfile candidate, create one
		let cfile_id = match cfile_id {
			Some(cfile_id) => cfile_id,
			None => Self::create(mm, CFileForCreate {}).await?,
		};

		// -- Now attach the cfile_id to this conv
		ConvBmc::update(
			mm,
			conv.id,
			ConvForUpdate {
				cfile_id: Some(cfile_id),
				..Default::default()
			},
		)
		.await?;

		Ok(cfile_id)
	}

	async fn find_candidate_for_space_id(mm: &ModelManager, space_id: Id) -> Result<Option<Id>> {
		// TODO: Might be better to order by conv.mtime or something like that
		let sql = r#"
SELECT conv.cfile_id
FROM conv
JOIN space ON conv.space_id = space.id
WHERE space.id = ?1 
ORDER BY conv.last_open DESC
LIMIT 1;		
		"#;
		let cfile_id: Option<Id> = mm.main_db().exec_returning_as(sql, (space_id,))?;

		Ok(cfile_id)
	}
}

// endregion: --- CFileBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_test_support::seed_space;
	use crate::model::conv::ConvForCreate;
	use crate::model::space::SpaceBmc;

	#[tokio::test]
	async fn test_get_or_create_for_conv() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let space_id_1 = seed_space(&mm, "test-space").await?;
		let conv_a = SpaceBmc::get_latest_conv(&mm, space_id_1).await?;
		let conv_id_b = ConvBmc::create(&mm, ConvForCreate::new(space_id_1)).await?;
		let conv_b = ConvBmc::get(&mm, conv_id_b).await?;

		// other space
		let space_id_2 = seed_space(&mm, "test-space-2").await?;
		let conv_c = SpaceBmc::get_latest_conv(&mm, space_id_2).await?;

		// -- Exec
		// get same conv, twice
		let cfile_a_1 = CFileBmc::getc_for_conv(&mm, &conv_a).await?;
		let cfile_a_2 = CFileBmc::getc_for_conv(&mm, &conv_a).await?;
		// another conv of same sapce
		let cfile_b_1 = CFileBmc::getc_for_conv(&mm, &conv_b).await?;
		// another conv of different space
		let cfile_c = CFileBmc::getc_for_conv(&mm, &conv_c).await?;

		// -- Check
		// should be the same
		assert_eq!(cfile_a_1.id, cfile_a_2.id, "Same conv should be the same cfile");
		assert_eq!(
			cfile_a_1.id, cfile_b_1.id,
			"Convs of same space should be the same cfile"
		);
		assert_ne!(
			cfile_a_1.id, cfile_c.id,
			"Convs of different spaces should not have same cfile"
		);

		Ok(())
	}
}

// endregion: --- Tests
