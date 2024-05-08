use crate::event::ModelEvent;
use crate::model::store::db_sqlite::SlDb;
use crate::model::Id;
use modql::SIden;
use sea_query::{DynIden, IntoIden, TableRef};
use serde::Serialize;

pub trait DbBmc {
	const TABLE: &'static str;

	fn table_ref() -> TableRef {
		TableRef::Table(SIden(Self::TABLE).into_iden())
	}

	fn table_iden() -> DynIden {
		SIden(Self::TABLE).into_iden()
	}

	/// Specifies that the table for this Bmc has timestamps (cid, ctime, mid, mtime) columns.
	/// This will allow the code to update those as needed.
	///
	/// default: true
	fn has_timestamps() -> bool {
		true
	}

	/// Speficies the entity has unique identifier
	/// Notes:
	///   - `uid` are stored as TEXT in sqlite.
	///   - Format is base58 (flicker) for maximum portability and simplity when used in file name suffixes and such.
	///   - 22 char long as they are generated from uuid (se lib-utils uuid_b58)
	/// NOTE: The strategy is to
	fn has_uid() -> bool {
		true
	}

	fn set_last_open_on_create() -> bool {
		false
	}

	fn get_entity_ref(entity_id: Id) -> EntityRef {
		EntityRef {
			rel: Self::TABLE,
			id: entity_id,
		}
	}

	// region:    --- Publish Event

	// Note: for now, we allow async_fn_in_trait, as this should be fine in this case.

	#[allow(async_fn_in_trait)]
	async fn publish_create_event(db: &SlDb, entity_id: Id) {
		db.publish(ModelEvent::Create(Self::get_entity_ref(entity_id))).await;
	}

	#[allow(async_fn_in_trait)]
	async fn publish_update_event(db: &SlDb, entity_id: Id) {
		db.publish(ModelEvent::Update(Self::get_entity_ref(entity_id))).await;
	}

	#[allow(async_fn_in_trait)]
	async fn publish_delete_event(db: &SlDb, entity_id: Id) {
		db.publish(ModelEvent::Delete(Self::get_entity_ref(entity_id))).await;
	}

	#[allow(async_fn_in_trait)]
	async fn publish_custom_event(db: &SlDb, custom: &'static str, entity_id: Id) {
		db.publish(ModelEvent::Custom(custom, Self::get_entity_ref(entity_id))).await;
	}

	// Note: for now, do not cimplete pub_custom_event, might not need it, we might remove variant.

	// endregion: --- Publish Event
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityRef {
	pub rel: &'static str,
	pub id: Id,
}
