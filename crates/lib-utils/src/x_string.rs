pub trait XStr {
	/// Returns the Some(string) if the string is not empty (after trime)
	fn x_non_empty_str(&self) -> Option<&str>;
}

impl XStr for Option<String> {
	fn x_non_empty_str(&self) -> Option<&str> {
		self.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty())
	}
}
