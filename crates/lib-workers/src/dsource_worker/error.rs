use derive_more::From;
use lib_core::model::Id;
use lib_core::{event, model};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	// -- dsource_add
	DItemHasNoFileMTime {
		ditem_id: Id,
	},
	DItemHasNoFileSize {
		ditem_id: Id,
	},

	// -- Libs
	#[from]
	Model(model::Error),
	#[from]
	Event(event::Error),
	#[from]
	Splitters(lib_splitters::Error),

	// -- Externals
	#[from]
	SimpleFs(simple_fs::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
