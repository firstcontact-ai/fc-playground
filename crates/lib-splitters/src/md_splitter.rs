use crate::{LinePart, LineProcessor};

// Core function to process the line.
fn md_part_from_line(content: String) -> LinePart {
	let is_title = content.starts_with('#');

	let level = if is_title {
		Some(content.chars().take_while(|&c| c == '#').count() as i64)
	} else {
		None
	};

	LinePart {
		is_title,
		level,
		content,
	}
}

// region:    --- Behavior Impl

// Struct to represent Markdown-specific behavior
pub struct MdSplitterLineProcessor;

// Implementing SplitterBehavior for Markdown
impl LineProcessor for MdSplitterLineProcessor {
	fn process_line(line_content: String) -> LinePart {
		md_part_from_line(line_content)
	}
}

// endregion: --- Behavior Impl
