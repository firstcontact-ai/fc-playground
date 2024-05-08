// region:    --- Modules

mod error;
mod main_db;
mod model_manager;
mod store;

// -- Flatten
pub use self::error::{Error, Result};
pub use base::{EntityRef, Id};
pub use cfile_db::*;
pub use main_db::*;
pub use model_manager::*;

pub(in crate::model) mod base;

pub mod cfile_db;
pub mod dfile_db;
pub mod support;

// endregion: --- Modules

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum DbType {
	Main,
	DFile,
	CFile,
}
