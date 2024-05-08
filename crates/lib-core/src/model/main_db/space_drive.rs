use crate::model::support::prelude::*;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[serde_as]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct SpaceDrive {
	pub id: Id,

	pub space_id: Id,
	pub drive_id: Id,
}

#[derive(Fields)]
pub struct SpaceDriveForCreateRec {
	pub space_id: Id,
	pub drive_id: Id,
	// flag to say that this
	pub space_default: bool,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct SpaceDriveFilter {
	pub id: Option<OpValsInt64>,

	pub space_id: Option<OpValsInt64>,
	pub drive_id: Option<OpValsInt64>,
	pub space_default: Option<OpValsBool>,
}

// endregion: --- Types

// region:    --- SpaceDriveBmc

pub struct SpaceDriveBmc;

impl DbBmc for SpaceDriveBmc {
	const TABLE: &'static str = "space_drive";

	fn has_uid() -> bool {
		false
	}
}

// endregion: --- SpaceDriveBmc
