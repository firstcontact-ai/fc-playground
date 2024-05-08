// region:    --- Modules

// -- Privates
mod error;
mod lfs;

// -- Flatten
pub use error::{Error, Result};

// -- Public
pub mod event;
pub mod model;

// -- Test
#[cfg(any(test, feature = "for-test"))]
pub mod _test_support;

// endregion: --- Modules

pub fn lib_core_demo() -> String {
	"Hello from lib-core".to_string()
}
