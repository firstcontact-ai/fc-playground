use crate::dsource_worker::{Error, Result};
use lib_core::event::DSourceEvent;
use lib_core::model::ditem::{DItem, DItemBmc, DItemForCreate, DItemForUpdate};
use lib_core::model::ditem_dsource::{DItemDSourceBmc, DItemDSourceForCreate};
use lib_core::model::dsource::DSourceBmc;
use lib_core::model::{Id, ModelManager};
use lib_utils::time::UnixTimeUs;
use simple_fs::SFile;

// TODO: Might revise visibility of this one (and perhaps have a #[cfg(test)])
pub async fn proc_dsource_added(mm: &ModelManager, dsource_id: Id) -> Result<()> {
	// -- Perform the work
	refresh_dsource_ditems(mm, dsource_id).await?;

	// -- Send event
	mm.hub().publish(DSourceEvent::DItemsRefreshed { dsource_id }).await;

	Ok(())
}

// region:    --- Internal

struct DItemCandidate {
	file_path: String,
	file_mtime: UnixTimeUs,
	file_size: i64,
	file_ext: String,
	folder_path: String,
}

impl DItemCandidate {
	/// If cannot do diff_path or get Modified, return None
	fn from_sfile(file: SFile) -> Option<DItemCandidate> {
		let file_path = file.to_string();
		let file_mtime = file.modified_us().unwrap_or_default().into(); // set to 0 if error.
		let file_size = file.file_size().ok()?;
		let folder_path = file.parent().map(|spath| spath.to_str().to_string()).unwrap_or_default();
		let file_ext = file.ext().to_string().to_lowercase();

		Some(DItemCandidate {
			file_path,
			file_size,
			file_mtime,
			folder_path,
			file_ext,
		})
	}
}

async fn refresh_dsource_ditems(mm: &ModelManager, dsource_id: Id) -> Result<()> {
	let dsource = DSourceBmc::get(mm, dsource_id).await?;
	let rref = &dsource.rref;
	// TODO: Probably need to do a cannonicalize of rref

	// -- Get local files - DItemCandidates from the `rref`
	// NOTE: For now only supports `.md` files
	let mut candidates: Vec<DItemCandidate> = simple_fs::iter_files(rref, Some(&["**/*.md"]), None)?
		.filter_map(DItemCandidate::from_sfile)
		.collect();
	candidates.sort_by(|a, b| a.file_path.cmp(&b.file_path));

	// -- Get the DItems from the DB
	let ditems = DItemBmc::list_for_base_path(mm, rref).await?;
	let ditem_by_full_path = ditems
		.into_iter()
		.map(|ditem| (ditem.file_path.clone(), ditem))
		.collect::<std::collections::HashMap<_, _>>();

	for candidate in candidates {
		let ditem = ditem_by_full_path.get(&candidate.file_path);

		let res = match ditem {
			Some(ditem) => udpate_ditem_if_needed(mm, ditem, candidate, dsource_id).await,
			None => create_ditem(mm, candidate, dsource_id).await,
		};
		if let Err(err) = res {
			println!("WARNING on create DItem: {err}");
		}
	}

	Ok(())
}

/// Update the ditem or ditem_dsource only if any of:
/// - `ditem.file_mtime < candidate.file_mtime`
/// - `ditem.file_size != candidate.file_mtime` (first bullet point should also change in this case)
async fn udpate_ditem_if_needed(
	mm: &ModelManager,
	ditem: &DItem,
	candidate: DItemCandidate,
	dsource_id: Id,
) -> Result<()> {
	let ditem_file_mtime = ditem.file_mtime.ok_or(Error::DItemHasNoFileMTime { ditem_id: ditem.id })?;
	let ditem_file_size = ditem.file_size.ok_or(Error::DItemHasNoFileMTime { ditem_id: ditem.id })?;

	// -- Compute the two property to potentially update
	let file_mtime = if ditem_file_mtime < candidate.file_mtime {
		Some(candidate.file_mtime)
	} else {
		None
	};
	let file_size = if ditem_file_size != candidate.file_size {
		Some(candidate.file_size)
	} else {
		None
	};

	// if we have something to update
	if file_mtime.is_some() || file_size.is_some() {
		// -- Update the ditem
		DItemBmc::update(
			mm,
			ditem.id,
			DItemForUpdate {
				file_mtime,
				file_size,
				..Default::default()
			},
		)
		.await?;

		// -- Update the mitem of the ditem_dsource
		DItemDSourceBmc::touch_from_pks(mm, ditem.id, dsource_id).await?;
	}

	Ok(())
}

/// Create a new ditem and the corresponsding
async fn create_ditem(mm: &ModelManager, candidate: DItemCandidate, dsource_id: Id) -> Result<()> {
	// TODO: Might want to create a transaction
	// -- Create the DItem
	let ditem_c = DItemForCreate {
		file_path: candidate.file_path,
		folder_path: candidate.folder_path,
		file_mtime: candidate.file_mtime,
		file_size: candidate.file_size,
		file_ext: candidate.file_ext,
	};
	let ditem_id = DItemBmc::create(mm, ditem_c).await?;

	// -- Create the corresponding DItemDSource
	let ditem_dsource_c = DItemDSourceForCreate { ditem_id, dsource_id };
	DItemDSourceBmc::create(mm, ditem_dsource_c).await?;

	Ok(())
}

// endregion: --- Internal

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use lib_core::_test_support::{seed_drive, seed_dsource};

	#[tokio::test]
	async fn test_proc_dsource_added() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_drive_id = seed_drive(&mm, "test_proc_dsource_added - drive 01").await?;
		let fx_dsource_id = seed_dsource(&mm, fx_drive_id, "../../test-data").await?;

		// -- Exec
		proc_dsource_added(&mm, fx_dsource_id).await?;
		proc_dsource_added(&mm, fx_dsource_id).await?;

		// -- Check
		let ditems = DItemBmc::list(&mm, None, None).await?;
		assert_eq!(ditems.len(), 3);
		let ditem_dsources = DItemDSourceBmc::list(&mm, None, None).await?;
		assert_eq!(ditem_dsources.len(), 3);

		// -- Debug
		// mm.main_db().print_table("ditem")?;
		// mm.main_db().print_table("ditem_dsource")?;

		Ok(())
	}
}

// endregion: --- Tests
