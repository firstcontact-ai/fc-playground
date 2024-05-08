// region:    --- Modules

#[allow(clippy::module_inception)]
mod conv_worker;
mod error;

pub use self::error::{Error, Result};
pub use conv_worker::*;

// endregion: --- Modules
