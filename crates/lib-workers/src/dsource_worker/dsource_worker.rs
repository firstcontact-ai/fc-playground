use crate::dsource_worker::processors::{proc_dfiles_refreshed, proc_ditems_refreshed, proc_dsource_added};
use crate::dsource_worker::Result;
use lib_core::event::{DSourceEvent, Subscriber};
use lib_core::model::ModelManager;
use tracing::debug;

pub struct DSourceWorker {
	mm: ModelManager,
}

impl DSourceWorker {
	pub fn start(mm: ModelManager) -> Result<()> {
		let ditemizer = DSourceWorker { mm };

		tokio::spawn(async move {
			let res = ditemizer.start_worker().await;
			match res {
				Ok(_) => println!("DSourceWorker ends OK"),
				Err(err) => println!("DSourceWorker ends ERROR {err:?}"),
			}
		});

		Ok(())
	}

	async fn start_worker(&self) -> Result<()> {
		debug!("STARTING");
		let mut sub: Subscriber<DSourceEvent> = self.mm.hub().subscriber()?;
		while let Ok(evt) = sub.next().await {
			debug!("EVT: {evt:?}");
			match evt {
				DSourceEvent::DSourceAdded { dsource_id } => proc_dsource_added(&self.mm, dsource_id).await?,
				DSourceEvent::DItemsRefreshed { dsource_id } => proc_ditems_refreshed(&self.mm, dsource_id).await?,
				DSourceEvent::DFilesRefreshed { dsource_id } => proc_dfiles_refreshed(&self.mm, dsource_id).await?,
				other => debug!("DSourceWorker event '{other:?}' not implemented yet"),
			}
		}

		debug!("ENDING");
		Ok(())
	}
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use lib_core::_test_support::seed_drive;
	use lib_core::model::dfile::DFileBmc;
	use lib_core::model::ditem::DItemBmc;
	use lib_core::model::drive::DriveBmc;
	use lib_core::model::dsource::DSourceForCreate;
	use lib_test_utils::sleep_ms;
	use simple_fs::SPath;
	use std::path::Path;
	use std::time::Duration;

	#[tokio::test]
	async fn test_dsource_worker_ok() -> Result<()> {
		// -- Setup & Fixtures
		// lib_utils::trace::init_trace();
		let mm = ModelManager::new().await?;
		DSourceWorker::start(mm.clone())?;
		let _fx_drive_name = "test_create_ok - drive 01";
		let fx_drive_id = seed_drive(&mm, "test_create_dsource_ok - drive 01").await?;
		let fx_dsource_rref = Path::new("../../test-data").canonicalize().map(SPath::from_path)??;

		// Note: Somehow we have to pause here, otherwise, the events are not received.
		//       We should investigate the reason.
		//       In prod code, not an issue, as the queues are started at start of the app.
		//       So there are an normal pause there.
		sleep_ms(1).await;

		// -- Exec
		let dsource_c = DSourceForCreate {
			rref: fx_dsource_rref.to_string(),
			drive_id: fx_drive_id,
			detail: None,
		};
		DriveBmc::add_dsource(&mm, dsource_c).await?;

		// we pause to let the worker work
		sleep_ms(1).await;

		// -- Check
		let ditems = DItemBmc::list(&mm, None, None).await?;
		assert_eq!(ditems.len(), 3);
		let dfiles = DFileBmc::list(&mm, None, None).await?;
		assert_eq!(dfiles.len(), 1);

		Ok(())
	}
}

// endregion: --- Tests
