use crate::model::drive::{DriveBmc, DriveForCreate};
use crate::model::dsource::DSourceForCreate;
use crate::model::space::{SpaceBmc, SpaceForCreate};
use crate::model::{Id, ModelManager};
use crate::Result;
use simple_fs::SPath;
use std::path::Path;

pub async fn seed_spaces(mm: &ModelManager, names: &[&str]) -> Result<Vec<Id>> {
	let mut ids = Vec::new();
	for name in names {
		let id = seed_space(mm, name).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_space(mm: &ModelManager, name: &str) -> Result<Id> {
	let id = SpaceBmc::create(
		mm,
		SpaceForCreate {
			name: name.to_string(),
			agent_id: None,
		},
	)
	.await?;
	Ok(id)
}

pub async fn seed_drives(mm: &ModelManager, names: &[&str]) -> Result<Vec<Id>> {
	let mut ids = Vec::new();
	for name in names {
		let id = seed_drive(mm, name).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_drive(mm: &ModelManager, name: &str) -> Result<Id> {
	let id = DriveBmc::create(mm, DriveForCreate { name: name.to_string() }).await?;
	Ok(id)
}

pub async fn seed_dsource(mm: &ModelManager, drive_id: Id, rref: &str) -> Result<Id> {
	let rref = Path::new(rref).canonicalize().map(SPath::from_path)??;

	// -- Exec
	let dsource_c = DSourceForCreate {
		rref: rref.into(),
		drive_id,
		detail: None,
	};
	let id = DriveBmc::add_dsource(mm, dsource_c).await?;

	Ok(id)
}
