use std::path::{Path, PathBuf};

/// Returns the application user dir where we store all of the database
/// and data information.
///
/// TODO: Needs to decide if this should be on `~/Documents` `~/.firstcontact`
///       or in more OS Centric app support common dir.
pub fn app_user_dir() -> PathBuf {
	Path::new(".firstcontact").to_path_buf()
}

// pub fn db_file_path(uid: &str) -> PathBuf {
// 	let file_name = format!("dfile-{uid}.db3");
// 	let dir = Path::new(".firstcontact").join("dfiles");
// 	dir.join(file_name).to_path_buf()
// }
