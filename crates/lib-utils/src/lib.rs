//! The utils module is designed to export independent sub-modules to the application code.
//!
//! Note: Even if the util sub-modules consist of a single file, they contain their own errors
//!       for improved compartmentalization.
//!
//!
//!

pub mod b64;
pub mod derive_commons;
pub mod envs;
pub mod hbs;
pub mod time;
pub mod trace;
pub mod uuid;
pub mod x_string;
pub mod x_value;
pub mod x_vec;

// region:    --- Macros

pub use std::format as f;

// Usage   `s!("Hello World")`
// Same as `"Hello World".to_string()`
#[macro_export]
macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}

// wrap some expression with ? into
// (|| first_branch.cond?.input?.is_json)()
// immediate execution.
#[macro_export]
macro_rules! o_wrap {
	($e:expr) => {{
		(|| $e)()
	}};
}
// endregion: --- Macros
