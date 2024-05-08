use crate::chain::InputContent;
use serde::Deserialize;
use serde_json::Value;
use serde_with::{serde_as, OneOrMany};

#[derive(Debug, Deserialize)]
pub struct CondNode {
	pub input: Option<CondInput>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CondInput {
	pub is_json: Option<bool>,
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	pub json_matches: Option<Vec<JsonMatch>>,
}

#[derive(Debug, Deserialize)]
pub struct JsonMatch {
	pub pointer: String,
	pub value: Value,
}

// region:    --- Node Activation Match

impl CondNode {
	pub fn matches_input(&self, content: &InputContent) -> bool {
		if let Some(when_input) = self.input.as_ref() {
			// -- Check if the is_json flag match
			if let Some(is_json) = when_input.is_json {
				if is_json != content.is_json() {
					return false;
				}
			}
			// -- Check the JsonMatch
			if let Some(json_matches) = when_input.json_matches.as_ref() {
				// if content not json, then, return false
				let InputContent::Json(input_value) = content else {
					return false;
				};

				// for each json match
				for json_match in json_matches {
					// if no value in input_value, then false
					let Some(pointed_value) = input_value.pointer(&json_match.pointer) else {
						return false;
					};

					if pointed_value != &json_match.value {
						return false;
					}
				}
			}
			true
		} else {
			false
		}
	}
}

// endregion: --- Node Activation Match
