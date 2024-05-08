use crate::model::dsource::{DSource, DSourceBmc, DSourceFilter, DSourceForCreate};
use crate::model::support::prelude::*;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[serde_as]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct Drive {
	pub id: Id,
	pub uid: String,

	pub name: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Fields, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct DriveForCreate {
	pub name: String,
}

#[derive(Fields, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct DriveForUpdate {
	pub name: Option<String>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct DriveFilter {
	pub id: Option<OpValsInt64>,

	pub name: Option<OpValsString>,
}

// endregion: --- Types

// region:    --- DriveBmc

pub struct DriveBmc;

impl DbBmc for DriveBmc {
	const TABLE: &'static str = "drive";
}

gen_mm_crud_fns!(
	Bmc: DriveBmc,
	ForGet: Drive,
	ForCreate: DriveForCreate,
	ForUpdate: DriveForUpdate,
	ForList: Drive,
	Filter: DriveFilter,
);

/// Implement the DSource accessors.
impl DriveBmc {
	pub async fn add_dsource(mm: &ModelManager, dsource_c: DSourceForCreate) -> Result<Id> {
		let dsource_c_rec = dsource_c.try_into()?;
		let id = DSourceBmc::create(mm, dsource_c_rec).await?;

		Ok(id)
	}

	pub async fn list_dsources(mm: &ModelManager, drive_id: Id) -> Result<Vec<DSource>> {
		let dsource_f = DSourceFilter {
			drive_id: Some((*drive_id).into()),
			..Default::default()
		};
		let dsources = DSourceBmc::list(mm, Some(vec![dsource_f]), None).await?;

		Ok(dsources)
	}
}

// endregion: --- DriveBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_test_support::seed_drives;
	use crate::model;
	use crate::model::dsource::DSourceKind;

	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_drive_name = "test_create_ok - drive 01";

		// -- Exec
		let drive_c = DriveForCreate {
			name: fx_drive_name.to_string(),
		};
		let drive_id = DriveBmc::create(&mm, drive_c).await?;

		// -- Check
		let drive = DriveBmc::get(&mm, drive_id).await?;
		assert_eq!(drive.name, fx_drive_name);

		Ok(())
	}

	#[tokio::test]
	async fn test_create_dsource_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_drive_id = seed_drives(&mm, &["test_create_dsource_ok - drive 01"]).await?[0];
		let fx_dsource_rref = "src/_test_support";

		// -- Exec
		let dsource_c = DSourceForCreate {
			rref: fx_dsource_rref.to_string(),
			drive_id: fx_drive_id,
			detail: None,
		};

		DriveBmc::add_dsource(&mm, dsource_c).await?;

		// -- Check
		let dsource = DriveBmc::list_dsources(&mm, fx_drive_id)
			.await?
			.pop()
			.ok_or("Does not have dsource")?;
		assert!(
			dsource.rref.ends_with(fx_dsource_rref),
			"rref should end with {fx_dsource_rref}"
		);
		// assert .kind is DSourceKind::Folder
		assert!(
			matches!(dsource.kind, DSourceKind::Folder),
			"DSourde.kind does not match Folder (but {:?})",
			dsource.kind
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_create_dsource_double_err() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_drive_id = seed_drives(&mm, &["test_create_dsource_ok - drive 01"]).await?[0];
		let fx_dsource_rref = "src/_test_support";

		// -- Exec
		let dsource_c = DSourceForCreate {
			rref: fx_dsource_rref.to_string(),
			drive_id: fx_drive_id,
			detail: None,
		};

		DriveBmc::add_dsource(&mm, dsource_c).await?;

		let dsource_c = DSourceForCreate {
			rref: fx_dsource_rref.to_string(),
			drive_id: fx_drive_id,
			detail: None,
		};

		let res = DriveBmc::add_dsource(&mm, dsource_c).await;

		// -- Check
		// match res to Err UniqueViolation
		assert!(res.is_err(), "should have a unique violation error");
		assert!(
			matches!(res.unwrap_err(), model::Error::UniqueViolation { .. }),
			"should have a unique violation error"
		);

		Ok(())
	}
}

// endregion: --- Tests
