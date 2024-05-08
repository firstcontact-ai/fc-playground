use crate::dsource_worker::Result;
use lib_core::event::DSourceEvent;
use lib_core::model::dfile::{DFile, DFileBmc};
use lib_core::model::dfile_db::ditem_ref::DItemRefBmc;
use lib_core::model::dfile_db::part::{PartBmc, PartForCreate};
use lib_core::model::ditem::{DItem, DItemBmc};
use lib_core::model::{Id, ModelManager};
use lib_splitters::SplitterKind;
use simple_fs::{get_buf_reader, SFile};
use std::collections::HashMap;

pub async fn proc_dfiles_refreshed(mm: &ModelManager, dsource_id: Id) -> Result<()> {
	// -- Perform work
	refresh_dsource_dfile_dbs(mm, dsource_id).await?;

	// -- Send Event
	mm.hub().publish(DSourceEvent::DFileDbsRefreshed { dsource_id }).await;

	Ok(())
}
// region:    --- Internal

async fn refresh_dsource_dfile_dbs(mm: &ModelManager, dsource_id: Id) -> Result<()> {
	let ditems = DItemBmc::list_ditems_to_proc_for_dsource(mm, dsource_id).await?;
	let dfiles = DFileBmc::list_dfiles_for_dsource(mm, dsource_id).await?;
	let dfiles_by_id: HashMap<i64, DFile> = dfiles.into_iter().map(|dfile| (*dfile.id, dfile)).collect();

	for ditem in ditems {
		let Some(dfile_id) = ditem.dfile_id.as_ref() else {
			println!(
				"WARNING - refresh_dsource_dfile_dbs - DItem {} does not have a a dfile_id",
				ditem.id
			);
			continue;
		};

		let Some(dfile) = dfiles_by_id.get(dfile_id) else {
			println!("WARNING - refresh_dsource_dfile_dbs - dfile_id {dfile_id} not found");
			continue;
		};

		update_db_file_parts(mm, &ditem, dfile).await?;
	}

	// region:    --- DEBUG
	// for (_, dfile) in dfiles_by_id.iter() {
	// 	let dfile_db = DFileBmc::get_dfile_db(mm, dfile).await?;
	// 	dfile_db.print_table("part");

	// 	let parts = PartBmc::content_search(&dfile_db, "material").await?;
	// 	for part in parts {
	// 		println!();
	// 		println!(
	// 			"part: {} (ditem_ref {}, line: {})\n{}",
	// 			part.id, part.ditem_ref_id, part.line_num, part.content
	// 		);
	// 	}
	// }
	// println!();
	// endregion: --- DEBUG

	Ok(())
}

async fn update_db_file_parts(mm: &ModelManager, ditem: &DItem, dfile: &DFile) -> Result<()> {
	// -- Get the dfile db
	let dfile_db = DFileBmc::get_dfile_db(mm, dfile).await?;

	// -- Create the `ditem_ref` row if not present
	let ditem_ref_id = DItemRefBmc::get_or_create_for_ditem_uid(&dfile_db, &ditem.uid).await?.id;
	let ditem_ref_id = *ditem_ref_id;

	// -- Create the parts
	let full_path = SFile::new(&ditem.file_path)?;
	let reader = get_buf_reader(full_path.path())?;

	let splitter_kind = SplitterKind::Md;
	let mut splitter_parts = lib_splitters::get_splitter_parts(splitter_kind, reader)?;

	while let Some(Ok(s_part)) = splitter_parts.next() {
		let part_c = PartForCreate {
			ditem_ref_id,
			is_title: s_part.is_title,
			level: s_part.level,
			group: s_part.group,
			line_num: s_part.line_num,
			content: s_part.content,
		};
		PartBmc::create(&dfile_db, part_c).await?;
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
	use crate::dsource_worker::processors::{proc_dfiles_refreshed, proc_ditems_refreshed, proc_dsource_added};
	use lib_core::_test_support::{seed_drive, seed_dsource};

	#[tokio::test]
	async fn test_proc_dfiles_refreshed() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_drive_id = seed_drive(&mm, "test_proc_dfiles_refreshed - drive 01").await?;
		let fx_dsource_id = seed_dsource(&mm, fx_drive_id, "../../test-data").await?;
		// update the ditems and dfiles
		proc_dsource_added(&mm, fx_dsource_id).await?;
		proc_ditems_refreshed(&mm, fx_dsource_id).await?;

		// -- Exec
		proc_dfiles_refreshed(&mm, fx_dsource_id).await?;

		// -- Check
		let dfile = DFileBmc::list(&mm, None, None)
			.await?
			.into_iter()
			.next()
			.ok_or("Should have a dfile")?;
		let dfile_db = DFileBmc::get_dfile_db(&mm, &dfile).await?;
		let distinct_ditem_ref_ids_count =
			dfile_db.exec_returning_num("select COUNT(distinct(ditem_ref_id)) as count from part", [])?;
		assert_eq!(distinct_ditem_ref_ids_count, 3);

		Ok(())
	}
}

// endregion: --- Tests
