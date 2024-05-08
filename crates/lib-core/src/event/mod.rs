//! The lib-core event module manages events related to the core model, allowing other
//! services (e.g., Workers) to listen to and subscribe to them.
//! It functions as the pub/sub hub.

// region:    --- Modules

mod error;
mod hub;
mod types;

pub use self::error::{Error, Result};

// -- Flatten
pub use hub::*;
pub use types::*;

// endregion: --- Modules
