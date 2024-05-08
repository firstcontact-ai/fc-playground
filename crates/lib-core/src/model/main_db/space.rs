use crate::model::agent::{Agent, AgentBmc};
use crate::model::conv::{Conv, ConvBmc, ConvFilter, ConvForCreate};
use crate::model::drive::{Drive, DriveBmc, DriveForCreate};
use crate::model::space_drive::{SpaceDrive, SpaceDriveBmc, SpaceDriveFilter, SpaceDriveForCreateRec};
use crate::model::support::prelude::*;
use modql::filter::{OpValBool, OpValInt64};

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[serde_as]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct Space {
	pub id: Id,
	pub uid: String,

	pub agent_id: Option<Id>,

	pub name: String,

	pub last_open: Option<i64>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Fields, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct SpaceForCreate {
	pub name: String,
	pub agent_id: Option<Id>,
}

#[derive(Fields, Deserialize, Default)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct SpaceForUpdate {
	pub name: Option<String>,
	pub agent_id: Option<Id>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct SpaceFilter {
	pub id: Option<OpValsInt64>,

	pub name: Option<OpValsString>,
	pub agent_id: Option<OpValsInt64>,
}

// endregion: --- Types

// region:    --- SpaceBmc

pub struct SpaceBmc;

impl DbBmc for SpaceBmc {
	const TABLE: &'static str = "space";

	fn set_last_open_on_create() -> bool {
		true
	}
}

gen_mm_crud_fns!(
	Bmc: SpaceBmc,
	ForGet: Space,
	ForUpdate: SpaceForUpdate,
	ForList: Space,
	Filter: SpaceFilter,
);

impl SpaceBmc {
	/// On Space create we create the following
	/// - The space
	/// - A drive that will be named "space-drive" and be attached to this space
	///   This will be the default drive for this space
	/// - A default conversation
	pub async fn create(mm: &ModelManager, space_c: SpaceForCreate) -> Result<Id> {
		let space_id = base::create::<Self, _>(mm.main_db(), space_c).await?;

		// -- Create Drive
		let drive_id = DriveBmc::create(
			mm,
			DriveForCreate {
				name: "space-drive".to_string(),
			},
		)
		.await?;
		Self::attach_drive(mm, space_id, drive_id, true).await?;

		// -- Create Conversation
		ConvBmc::create(
			mm,
			ConvForCreate {
				title: Some("First Conversation".to_string()),
				space_id,
			},
		)
		.await?;

		Ok(space_id)
	}

	pub async fn get_latest(mm: &ModelManager) -> Result<Space> {
		let list_options = ListOptions {
			limit: Some(1),
			offset: None,
			order_bys: Some("!last_open".into()),
		};

		let space: Option<Space> = SpaceBmc::first(mm, None, Some(list_options)).await?;

		space.ok_or(Error::NoSpaceFound)
	}

	/// Return the latest Conv
	pub async fn get_latest_conv(mm: &ModelManager, space_id: Id) -> Result<Conv> {
		let list_options = ListOptions {
			limit: Some(1),
			offset: None,
			order_bys: Some("!last_open".into()),
		};

		let space_id: i64 = space_id.into();

		let conv: Option<Conv> = ConvBmc::first(
			mm,
			Some(vec![ConvFilter {
				space_id: Some(space_id.into()),
				..Default::default()
			}]),
			Some(list_options),
		)
		.await?;

		conv.ok_or(Error::NoConvFoundForSpace {
			space_id: space_id.into(),
		})
	}

	pub async fn attach_drive(mm: &ModelManager, space_id: Id, drive_id: Id, space_default: bool) -> Result<()> {
		let _space_drive_id = base::create::<SpaceDriveBmc, _>(
			mm.main_db(),
			SpaceDriveForCreateRec {
				space_id,
				drive_id,
				space_default,
			},
		)
		.await?;

		Ok(())
	}

	pub async fn get_default_drive(mm: &ModelManager, space_id: Id) -> Result<Drive> {
		let space_drive: SpaceDrive = base::first::<SpaceDriveBmc, _, _>(
			mm.main_db(),
			Some(SpaceDriveFilter {
				space_id: Some(OpValInt64::Eq(*space_id).into()),
				space_default: Some(OpValBool::Eq(true).into()),
				..Default::default()
			}),
			None,
		)
		.await?
		.ok_or(Error::NoDefaultSpaceDriveFound)?;

		let drive = DriveBmc::get(mm, space_drive.drive_id).await?;

		Ok(drive)
	}

	// region:    --- Agent Related

	pub async fn set_agent(mm: &ModelManager, space_id: Id, agent_id: Id) -> Result<()> {
		let space = SpaceBmc::get(mm, space_id).await?;
		let space_update = SpaceForUpdate {
			agent_id: Some(agent_id),
			..Default::default()
		};

		SpaceBmc::update(mm, space.id, space_update).await?;

		Ok(())
	}

	/// Note: using the `seek_` prefix, because, it return an Option, and does not fail if no agent.
	//        But will fail if space is not found.
	pub async fn seek_agent(mm: &ModelManager, space_id: Id) -> Result<Option<Agent>> {
		let space = SpaceBmc::get(mm, space_id).await?;
		let Some(agent_id) = space.agent_id else {
			return Ok(None);
		};

		let agent = AgentBmc::get(mm, agent_id).await?;
		Ok(Some(agent))
	}

	// endregion: --- Agent Related
}

// endregion: --- SpaceBmc

// region:    --- SpaceError

pub type SpaceResult<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum SpaceError {}

// region:    --- Error Boilerplate

impl core::fmt::Display for SpaceError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for SpaceError {}

// endregion: --- Error Boilerplate

// endregion: --- SpaceError

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_test_support::seed_spaces;
	use lib_utils::s;

	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_name = s!("test-space 01");

		// -- Exec
		let space_id = SpaceBmc::create(
			&mm,
			SpaceForCreate {
				name: fx_name,
				agent_id: None,
			},
		)
		.await?;

		// -- Check
		let spaces = SpaceBmc::list(&mm, None, None).await?;
		assert_eq!(spaces.len(), 1);
		assert_eq!(*space_id, 1);
		// check we have a default drive (if we get it, it's enough)
		let _default_drive = SpaceBmc::get_default_drive(&mm, space_id).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_get_latest_space_ok_from_already_created() -> Result<()> {
		// -- Setup & Fixtures
		let mm = ModelManager::new().await?;
		let fx_names = &[
			"test_get_latest_space_ok_from_already_created 01",
			"test_get_latest_space_ok_from_already_created 02",
		];
		let _ids = seed_spaces(&mm, fx_names).await?;

		// -- Exec
		let latest_space = SpaceBmc::get_latest(&mm).await?;

		// -- Check
		// should the the latest ones
		assert_eq!(latest_space.name, fx_names[1]);
		// make sure that 2 got created
		let spaces = SpaceBmc::list(&mm, None, None).await?;
		assert_eq!(spaces.len(), 2);

		Ok(())
	}
}

// endregion: --- Tests
