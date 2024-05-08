// region:    --- Modules

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

mod agent_defs;
mod mock_utils;
mod print;
mod seed_for_runner;

pub use mock_utils::*;
pub use print::*;
pub use seed_for_runner::*;

// endregion: --- Modules
