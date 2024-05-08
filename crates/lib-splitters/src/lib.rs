// region:    --- Modules

mod error;
pub use error::{Error, Result};

mod splitter;

// -- Flatten
pub use splitter::*;
use std::io::BufRead;

pub mod md_splitter;

// endregion: --- Modules

pub fn get_splitter_parts<B: BufRead>(kind: SplitterKind, reader: B) -> Result<SplitterParts<B>> {
	Ok(SplitterParts::new(reader, kind))
}
