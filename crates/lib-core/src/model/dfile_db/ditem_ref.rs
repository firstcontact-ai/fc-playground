use crate::model::support::prelude::*;
use lib_utils::time::UnixTimeUs;

// region:    --- Types

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct DItemRef {
	pub id: Id,

	ditem_uid: String,

	pub ctime: UnixTimeUs,
	pub mtime: UnixTimeUs,
}

#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct DItemRefForCreate {
	ditem_uid: String,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct DItemRefFilter {
	pub id: Option<OpValsInt64>,

	pub ditem_uid: Option<OpValsString>,
}

// endregion: --- Types

// region:    --- Bmc

pub struct DItemRefBmc;

impl DbBmc for DItemRefBmc {
	const TABLE: &'static str = "ditem_ref";

	fn has_uid() -> bool {
		false
	}
}

generate_sldb_crud_fns!(
	Bmc: DItemRefBmc,
	ForGet: DItemRef,
	ForGetByUid: DItemRef,
	ForCreate: DItemRefForCreate,
	ForList: DItemRef,
	Filter: DItemRefFilter,
);

impl DItemRefBmc {
	pub async fn get_or_create_for_ditem_uid(db: &SlDb, ditem_uid: &str) -> Result<DItemRef> {
		// TODO: Might want to do an upsert here
		let ditem_ref = DItemRefBmc::first(
			db,
			Some(vec![DItemRefFilter {
				ditem_uid: Some(ditem_uid.into()),
				..Default::default()
			}]),
			None,
		)
		.await?;

		let ditem_ref = match ditem_ref {
			Some(ditem_ref) => Ok(ditem_ref),
			None => {
				let ditem_ref_id = DItemRefBmc::create(
					db,
					DItemRefForCreate {
						ditem_uid: ditem_uid.to_string(),
					},
				)
				.await?;
				DItemRefBmc::get(db, ditem_ref_id).await
			}
		}?;

		Ok(ditem_ref)
	}
}

// endregion: --- Bmc
