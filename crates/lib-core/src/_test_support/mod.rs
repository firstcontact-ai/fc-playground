#![allow(unused)] // For early development.

// region:    --- Modules

mod seeders;

// -- Flatten
pub use seeders::*;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

// endregion: --- Modules
