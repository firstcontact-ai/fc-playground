// region:    --- InputContent

use serde_json::{from_str, Value};

/// The content sent by the AI or User to be matched against
/// the UI
#[derive(Debug, Clone)]
pub enum InputContent {
	Text(String),
	Json(Value),
}

// implement from String
impl From<String> for InputContent {
	fn from(value: String) -> Self {
		InputContent::new(value)
	}
}

// implement from &str
impl From<&str> for InputContent {
	fn from(value: &str) -> Self {
		InputContent::new(value)
	}
}

// implements from &String
impl From<&String> for InputContent {
	fn from(value: &String) -> Self {
		InputContent::new(value)
	}
}

// implement display
impl std::fmt::Display for InputContent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			InputContent::Text(value) => write!(f, "{}", value),
			InputContent::Json(value) => write!(f, "{}", value),
		}
	}
}

impl InputContent {
	pub fn new(content: impl Into<String>) -> Self {
		let content = content.into();
		match from_str(&content) {
			Ok(value) => InputContent::Json(value),
			Err(_) => InputContent::Text(content),
		}
	}

	pub fn is_json(&self) -> bool {
		matches!(self, InputContent::Json(_))
	}
}

// endregion: --- InputContent
