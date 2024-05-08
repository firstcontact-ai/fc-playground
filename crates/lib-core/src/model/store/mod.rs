// region:    --- Modules

mod error;

// -- Flatten
pub use self::error::{Error, Result};

pub mod db_sqlite;
pub mod sea_utils;

// endregion: --- Modules
