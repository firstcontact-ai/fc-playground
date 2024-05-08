use crate::dsource_worker::Result;
use lib_core::event::DSourceEvent;
use lib_core::model::dfile::DFileBmc;
use lib_core::model::ditem::{DItemBmc, DItemForUpdate};
use lib_core::model::{Id, ModelManager};

/// Refresh the `DFile` records in the main db for a given `DSource` (create as needed)
/// Note: Does not refresh the dfile_dbs, this is done by the `refresh_dsource_dfile_dbs`
pub async fn proc_ditems_refreshed(mm: &ModelManager, dsource_id: Id) -> Result<()> {
	// -- Perform work
	refresh_dsource_dfiles(mm, dsource_id).await?;

	// -- Send Event
	mm.hub().publish(DSourceEvent::DFilesRefreshed { dsource_id }).await;

	Ok(())
}

// region:    --- Internal

async fn refresh_dsource_dfiles(mm: &ModelManager, dsource_id: Id) -> Result<()> {
	let ditems = DItemBmc::list_ditems_without_dfile_for_dsource(mm, dsource_id).await?;

	for ditem in ditems {
		// -- get the dfile_db
		let _dfile = match ditem.dfile_id {
			Some(dfile_id) => DFileBmc::get(mm, dfile_id).await?,
			None => {
				// Get or create the dfile based on the dsource
				let dfile = DFileBmc::get_or_create_for_dsource(mm, dsource_id).await?;
				// Update this DItem with this dfile
				DItemBmc::update(
					mm,
					ditem.id,
					DItemForUpdate {
						dfile_id: Some(dfile.id.into()),
						..Default::default()
					},
				)
				.await?;
				dfile
			}
		};
	}

	Ok(())
}

// endregion: --- Internal

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::dsource_worker::processors::proc_dsource_added;
	use lib_core::_test_support::{seed_drive, seed_dsource};
	use lib_core::model::ditem_dsource::DItemDSourceBmc;

	#[tokio::test]
	async fn test_proc_ditems_refreshed() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_drive_id = seed_drive(&mm, "proc_ditems_refreshed - drive 01").await?;
		let fx_dsource_id = seed_dsource(&mm, fx_drive_id, "../../test-data").await?;
		proc_dsource_added(&mm, fx_dsource_id).await?;

		// -- Exec
		proc_ditems_refreshed(&mm, fx_dsource_id).await?;

		// -- Check
		let ditems = DItemBmc::list(&mm, None, None).await?;
		assert_eq!(ditems.len(), 3);
		let ditem_dsources = DItemDSourceBmc::list(&mm, None, None).await?;
		assert_eq!(ditem_dsources.len(), 3);
		let dfiles = DFileBmc::list(&mm, None, None).await?;
		assert_eq!(dfiles.len(), 1);

		Ok(())
	}
}

// endregion: --- Tests
