// region:    --- Modules

#[allow(clippy::module_inception)]
mod dsource_worker;
mod error;

pub use dsource_worker::*;
pub use error::{Error, Result};

pub mod processors;

// endregion: --- Modules
