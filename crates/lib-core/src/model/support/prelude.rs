//! Prelude for all of the entity type/bmc modules

pub use crate::gen_mm_crud_fns;
pub use crate::generate_sldb_crud_fns;
pub(in crate::model) use crate::model::base;
pub use crate::model::base::CommonIden;
pub use crate::model::base::{DbBmc, Id};
pub use crate::model::store::db_sqlite::SlDb;
pub use crate::model::{Error, ModelManager, Result};
pub use lib_utils::time::Rfc3339;
pub use lib_utils::time::UnixTimeUs;
pub use modql::field::Fields;
pub use modql::field::HasFields;
pub use modql::field::SeaFieldValue;
pub use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
pub use modql::FromSqliteRow;
pub use modql::FromSqliteValue;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{serde_as, skip_serializing_none};
pub use time::OffsetDateTime;
