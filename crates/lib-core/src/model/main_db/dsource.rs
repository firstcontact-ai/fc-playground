use crate::event::DSourceEvent;
use crate::model::support::prelude::*;
use derive_more::From;
use modql::field::SeaFieldValue;
use modql::FromSqliteValue;
use sea_query::Iden;
use serde_with::{serde_as, DisplayFromStr};
use simple_fs::SPath;
use std::path::Path;

// region:    --- Types

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, SeaFieldValue, FromSqliteValue, Serialize, Deserialize)]
pub enum DSourceKind {
	File,
	Folder,
	GhRepo,
	GgXls,
	GgDoc,
	MsXls,
	MsDoc,
	Unknown,
}

impl DSourceKind {
	/// TODO: Needs to support more than local files
	pub fn from_rref(rref: &str) -> DSourceResult<DSourceKind> {
		if rref.contains(':') {
			Err(DSourceError::DSourceRemoteRrefNotSupported { rref: rref.to_string() })?;
		}

		let path = SPath::new(rref)?;

		if path.is_dir() {
			Ok(DSourceKind::Folder)
		} else if path.is_file() {
			Ok(DSourceKind::File)
		} else {
			Err(DSourceError::DSourceFileNotFound { rref: rref.to_string() })?
		}
	}
}

#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Fields, FromSqliteRow, Serialize, Deserialize)]
pub struct DSource {
	pub id: Id,
	pub uid: String,

	pub drive_id: Id,

	pub kind: DSourceKind,

	pub name: String,
	pub rref: String,

	pub detail: Option<serde_json::Value>,
}

#[derive(Fields, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct DSourceForCreate {
	pub drive_id: Id,
	pub rref: String,
	pub detail: Option<serde_json::Value>,
}

#[derive(Debug, Fields, Deserialize)]
pub(in crate::model) struct DSourceForCreateRec {
	pub drive_id: Id,
	pub name: String,
	pub rref: String,
	pub kind: DSourceKind,
	pub detail: Option<serde_json::Value>,
}

impl TryInto<DSourceForCreateRec> for DSourceForCreate {
	type Error = DSourceError;

	fn try_into(self) -> DSourceResult<DSourceForCreateRec> {
		// TODO: Needs to support other data sources than file
		// compute the kind
		let kind = DSourceKind::from_rref(&self.rref)?;

		let (name, rref) = match kind {
			DSourceKind::File | DSourceKind::Folder => {
				let rref =
					Path::new(&self.rref)
						.canonicalize()
						.map_err(|_| DSourceError::DSourceFileFailCannonicalize {
							rref: self.rref.to_string(),
						})?;
				// get the SPath to make sure all utf8
				let spath = SPath::new(rref)?;
				(spath.file_name().to_string(), spath.to_string())
			}
			_ => return Err(DSourceError::KindNotSupported { rref: self.rref, kind }),
		};

		Ok(DSourceForCreateRec {
			drive_id: self.drive_id,
			name,
			rref,
			kind,
			detail: self.detail,
		})
	}
}

/// Note 1: Here, we do not want to update the "rref" as we might already have
///         some data on disk with the DSource.uid, and this will cause problems.
/// Note 2: However, as of now, it's not clear if a DSource will have a local folder/file
///         or if it is the `DItem` or `DFile` that will have those.
#[derive(Fields, Deserialize)]
#[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
pub struct DSourceForUpdate {
	detail: Option<serde_json::Value>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct DSourceFilter {
	pub id: Option<OpValsInt64>,

	pub drive_id: Option<OpValsInt64>,

	pub name: Option<OpValsString>,
	pub rref: Option<OpValsString>,
}

#[derive(Iden)]
pub enum DSourceIden {
	DsourceId,
}

// endregion: --- Types

// region:    --- DSourceBmc

pub struct DSourceBmc;

impl DbBmc for DSourceBmc {
	const TABLE: &'static str = "dsource";
}

gen_mm_crud_fns!(
	Bmc: DSourceBmc,
	ForGet: DSource,
	ForUpdate: DSourceForUpdate,
	ForList: DSource,
	Filter: DSourceFilter,
);

impl DSourceBmc {
	// Implemented manually since it has different visibility
	pub(in crate::model) async fn create(mm: &ModelManager, entity_c: DSourceForCreateRec) -> Result<Id> {
		let dsource_id = base::create::<Self, _>(mm.main_db(), entity_c).await?;
		mm.hub().publish(DSourceEvent::DSourceAdded { dsource_id }).await;
		Ok(dsource_id)
	}
}

// endregion: --- DSourceBmc

// region:    --- DSourceError

type DSourceResult<T> = core::result::Result<T, DSourceError>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum DSourceError {
	DSourceRemoteRrefNotSupported {
		rref: String,
	},
	DSourceFileNotFound {
		rref: String,
	},

	DSourceFileFailCannonicalize {
		rref: String,
	},

	KindNotSupported {
		rref: String,
		kind: DSourceKind,
	},

	#[from]
	SimpleFs(#[serde_as(as = "DisplayFromStr")] simple_fs::Error),
}

impl core::fmt::Display for DSourceError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for DSourceError {}

// endregion: --- DSourceError
