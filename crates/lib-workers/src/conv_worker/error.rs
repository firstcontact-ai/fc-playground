use derive_more::From;
use lib_core::event;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	// -- Libs
	#[from]
	Event(event::Error),
	#[from]
	Model(lib_core::model::Error),
	#[from]
	Ais(lib_ais::Error),

	// -- Externals
	#[from]
	Io(std::io::Error), // as example
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
