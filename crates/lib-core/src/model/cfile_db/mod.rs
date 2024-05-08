//! `cfile_db` are where the content of the `CFile` are stored:
//!
//! - `CFile` belong to the main db.
//!
//! and the `cfile_db` has the following entities:
//!
//! - `cfile_ref` - which is a ref to the main-db `cfile`
//! - `msg`       - message from the user, AI, or logic (app logic)

//! NOTE: Might want to have the modules below only for the (in crate::model) scope
//!       but right now, the ais runner needs to have precise control
//!       of the MsgStep

// region:    --- Modules

pub mod conv_ref;
pub mod msg;
pub mod stack_step;

// endregion: --- Modules
