use crate::model::conv::{Conv, ConvBmc, ConvForUpdate};
use crate::model::support::prelude::*;
use lib_utils::time::UnixTimeUs;

// region:    --- Types

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct ConvRef {
	pub id: Id,

	conv_uid: String,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct ConvRefForCreate {
	conv_uid: String,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct ConvRefFilter {
	pub id: Option<OpValsInt64>,

	pub conv_uid: Option<OpValsString>,
}

// endregion: --- Types

// region:    --- Bmc

pub struct ConvRefBmc;

impl DbBmc for ConvRefBmc {
	const TABLE: &'static str = "conv_ref";

	fn has_uid() -> bool {
		false
	}
}

generate_sldb_crud_fns!(
	Bmc: ConvRefBmc,
	ForGet: ConvRef,
	ForGetByUid: ConvRef,
	ForCreate: ConvRefForCreate,
	ForList: ConvRef,
	Filter: ConvRefFilter,
);

impl ConvRefBmc {
	pub async fn get_or_create_conv_ref_id(mm: &ModelManager, db: &SlDb, conv: &Conv) -> Result<Id> {
		// -- If we have a conv_ref_id, then, return it.
		if let Some(conv_ref_id) = conv.conv_ref_id {
			// TODO: Might want to do a check of the `conv_ref.conv_uid` row match
			//       this `conv.uid`. Could be a fast lookup, just return `.conv_uid`
			return Ok(conv_ref_id);
		}

		// -- Otherwise, we create it and add it to the conv
		// create the conv_ref
		let conv_ref_id = ConvRefBmc::create(
			db,
			ConvRefForCreate {
				conv_uid: conv.uid.to_string(),
			},
		)
		.await?;
		// we update the conv with the conv_ref_id for faster lookup
		ConvBmc::update(
			mm,
			conv.id,
			ConvForUpdate {
				conv_ref_id: Some(conv_ref_id),
				..Default::default()
			},
		)
		.await?;

		Ok(conv_ref_id)
	}
}

// endregion: --- Bmc
