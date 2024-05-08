//! `dfile_db` module is for the "data file" content of the `dfile` contained in the `main-db`.
//!
//! It contains the following entities:
//!
//! - `ditem_ref`: Which is a pointer to the main_db `ditem`. Typically a file.
//! - `part`: Which is a part of a of a ditem (i.e. file).

// region:    --- Modules

pub mod ditem_ref;
pub mod part;

// endregion: --- Modules
