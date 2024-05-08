use crate::event::Hub;
use crate::event::{ModelEvent, Publisher};
use crate::model::store::db_sqlite::{self, init_db, SlDb};
use crate::model::{DbType, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type ModelPublisher = Publisher<ModelEvent>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct SecondaryDbKey {
	db_type: DbType,
	uid: String,
}

#[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Clone)]
pub struct ModelManager {
	hub: Hub,
	main_db: SlDb,

	sec_dbs: Arc<Mutex<HashMap<SecondaryDbKey, SlDb>>>,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let hub = Hub::default();
		let model_publisher: ModelPublisher = hub.publisher()?;
		let main_db = db_sqlite::init_db(&DbType::Main, model_publisher, None)?;

		Ok(Self {
			hub,
			main_db,
			sec_dbs: Default::default(),
		})
	}

	pub fn hub(&self) -> &Hub {
		&self.hub
	}

	pub fn main_db(&self) -> &SlDb {
		&self.main_db
	}

	pub(in crate::model) async fn dfile_db(&self, uid: &str) -> Result<SlDb> {
		let db = self.sec_db(DbType::DFile, uid).await?;
		Ok(db)
	}

	pub(in crate::model) async fn cfile_db(&self, uid: &str) -> Result<SlDb> {
		let db = self.sec_db(DbType::CFile, uid).await?;
		Ok(db)
	}

	/// Return (create/init if needed) a secondary db
	async fn sec_db(&self, db_type: DbType, uid: &str) -> Result<SlDb> {
		let mut sec_dbs = self.sec_dbs.lock().await;
		let key = SecondaryDbKey {
			db_type,
			uid: uid.to_string(),
		};

		let db = match sec_dbs.get(&key) {
			Some(db) => db.clone(),
			None => {
				let model_publisher: ModelPublisher = self.hub().publisher()?;
				let db = init_db(&key.db_type, model_publisher, Some(&key.uid))?;
				sec_dbs.insert(key, db.clone());
				db
			}
		};

		Ok(db)
	}
}
