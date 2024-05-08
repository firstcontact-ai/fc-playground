use crate::md_splitter::MdSplitterLineProcessor;
use crate::Result;
use std::io::{BufRead, Lines};

// region:    --- Processor

pub struct LinePart {
	pub is_title: bool,
	pub level: Option<i64>,
	pub content: String,
}
pub trait LineProcessor {
	fn process_line(line_content: String) -> LinePart;
}

// endregion: --- Processor

// region:    --- Types

pub struct SplitPart {
	pub is_title: bool,
	pub level: i64,
	pub group: i64,
	pub line_num: i64,
	pub content: String,
}

pub enum SplitterKind {
	Md,
}

// endregion: --- Types

// region:    --- Kind / ProcessorDispatch

impl SplitterKind {
	// Note: here we do not implement the SplitterProcessor, because not needed, and we need the &self
	//       For the SplitterProcessor, we do not need &self, otherwise, we will have to have uncessary states
	fn process_line(&self, line_content: String) -> LinePart {
		match self {
			SplitterKind::Md => MdSplitterLineProcessor::process_line(line_content),
		}
	}
}

// endregion: --- Kind / ProcessorDispatch

pub struct SplitterParts<B: BufRead> {
	kind: SplitterKind,
	// -- The read buffer of the lines
	lines: Lines<B>,
	// -- The states for building the parts
	last_line_num: i64,
	last_level: i64,
	last_group: i64,
}

impl<B: BufRead> SplitterParts<B> {
	pub fn new(reader: B, kind: SplitterKind) -> Self {
		SplitterParts {
			lines: reader.lines(),
			last_line_num: 0,
			last_level: 0,
			last_group: 0,
			kind,
		}
	}
}

impl<B: BufRead> Iterator for SplitterParts<B> {
	type Item = Result<SplitPart>;

	fn next(&mut self) -> Option<Result<SplitPart>> {
		// TODO: Probably needs to implement when Error occurs.
		//       While we never return Some(Err..) as this point, we might later,
		//       hence, the iterator siganture
		match self.lines.next() {
			Some(Ok(line_content)) => {
				// -- process line
				let line_part = self.kind.process_line(line_content);

				// -- Build part
				// compute the new level/group
				// Note: if this LinePart has a level, then, it's a new one (even if same number)
				//       so, we create a new group
				let (level, group) = if let Some(new_level) = line_part.level {
					(new_level, self.last_group + 1)
				} else {
					(self.last_level, self.last_group)
				};

				let line_num = self.last_line_num + 1;

				let part = SplitPart {
					is_title: line_part.is_title,
					level,
					group,
					line_num,
					content: line_part.content,
				};

				// -- Update the iterator states
				self.last_line_num = line_num;
				self.last_group = group;
				self.last_level = level;

				// -- Return result
				Some(Ok(part))
			}
			_ => None,
		}
	}
}
