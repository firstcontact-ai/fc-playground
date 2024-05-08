use crate::model::{EntityRef, Id};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum DSourceEvent {
	// When a dsource is added.
	DSourceAdded { dsource_id: Id },
	// When all of the ditems of a dsource have been refreshed (created or updated)
	DItemsRefreshed { dsource_id: Id },
	// When all of the dfiles of a dsource have been udpate
	DFilesRefreshed { dsource_id: Id },
	// When all of the dfile dbs of a dsource have been udpated
	DFileDbsRefreshed { dsource_id: Id },
}

// NOTE: might go with the `Evt` suffix to avoid overloading `Msg`
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ConvEvent {
	ConvWorkNew { conv_id: Id },
	ConvWorkDone { conv_id: Id },
}

impl ConvEvent {
	pub fn name(&self) -> &'static str {
		match self {
			Self::ConvWorkNew { .. } => "conv_work_new",
			Self::ConvWorkDone { .. } => "conv_work_done",
		}
	}
}

#[derive(Debug, Clone, Serialize)]
pub enum ModelEvent {
	Create(EntityRef),
	Update(EntityRef),
	Delete(EntityRef),
	Custom(&'static str, EntityRef),
}

impl ModelEvent {
	pub fn action_name(&self) -> &'static str {
		match self {
			Self::Create(_) => "create",
			Self::Update(_) => "update",
			Self::Delete(_) => "delete",
			Self::Custom(name, _) => name,
		}
	}

	pub fn entity_ref(&self) -> &EntityRef {
		match self {
			Self::Create(rf) => rf,
			Self::Update(rf) => rf,
			Self::Delete(rf) => rf,
			Self::Custom(_, rf) => rf,
		}
	}
}
