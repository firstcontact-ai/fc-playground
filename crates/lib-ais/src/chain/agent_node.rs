use crate::chain::{CondNode, InputContent};
use core::fmt;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Default)]
pub struct AgentNode {
	#[serde(default)]
	pub agent: AgentRef,
	pub name_input: Option<String>,
	pub name_output: Option<String>,
	pub when: Option<CondNode>,
}

impl AgentNode {
	#[allow(unused)] // For early development.
	pub fn should_activate(&self, input_content: &InputContent) -> bool {
		match self.when.as_ref() {
			Some(when_activation) => when_activation.matches_input(input_content),
			None => true,
		}
	}
}

#[derive(Debug, Default)]
pub enum AgentRef {
	#[default]
	Same, // self, default
	Uid(String),
	Name(String),
	Id(i64),
}

// region:    --- AgentRef Deserializer

struct AgentRefVisitor;

impl<'de> Visitor<'de> for AgentRefVisitor {
	type Value = AgentRef;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("!!an object with a 'self', 'uid', or 'id' field")
	}

	fn visit_str<E>(self, v: &str) -> Result<AgentRef, E>
	where
		E: serde::de::Error,
	{
		if v == "self" {
			Ok(AgentRef::Same)
		} else {
			Err(E::custom("agent string value is invalid. Can only be 'self'"))
		}
	}

	fn visit_map<V>(self, mut map: V) -> Result<AgentRef, V::Error>
	where
		V: MapAccess<'de>,
	{
		let mut agent_ref = AgentRef::Same; // default
		while let Some(key) = map.next_key::<String>()? {
			match key.as_str() {
				"uid" => {
					let uid: String = map.next_value()?;
					agent_ref = AgentRef::Uid(uid);
					break; // Assuming only one of these keys is present
				}
				"id" => {
					let id: i64 = map.next_value()?;
					agent_ref = AgentRef::Id(id);
					break; // Assuming only one of these keys is present
				}
				"name" => {
					let name: String = map.next_value()?;
					agent_ref = AgentRef::Name(name);
					break; // Assuming only one of these keys is present
				}
				_ => {
					let _: serde::de::IgnoredAny = map.next_value()?; // Ignore any unknown fields
				}
			}
		}
		Ok(agent_ref)
	}
}

impl<'de> Deserialize<'de> for AgentRef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(AgentRefVisitor)
	}
}

// endregion: --- AgentRef Deserializer

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;

	#[test]
	fn test_agent_node_deserialize_with_uid_agent() -> Result<()> {
		const AGENT_NODE: &str = r#"
{
	"agent": { "uid": "some_uid" }
}
"#;
		// -- Exec
		let agent_node: AgentNode = serde_json::from_str(AGENT_NODE)?;

		// -- Check
		match agent_node.agent {
			AgentRef::Uid(uid) => assert_eq!(uid, "some_uid"),
			_ => return Err("should have been uid".into()),
		}

		Ok(())
	}

	#[test]
	fn test_agent_node_deserialize_with_no_agent() -> Result<()> {
		// -- Fixtures
		const SIMPLE_AGENT_NODE_SELF: &str = r#"{}"#;

		// -- Exec
		let agent_node: AgentNode = serde_json::from_str(SIMPLE_AGENT_NODE_SELF)?;

		// -- Check
		match agent_node.agent {
			AgentRef::Same => (),
			_ => return Err("should have been same".into()),
		}

		Ok(())
	}

	#[test]
	fn test_agent_node_deserialize_with_agent_self() -> Result<()> {
		// -- Fixtures
		const SIMPLE_AGENT_NODE_SELF: &str = r#"
{
	"agent": "self"
}
"#;

		// -- Exec
		let agent_node: AgentNode = serde_json::from_str(SIMPLE_AGENT_NODE_SELF)?;

		// -- Check
		match agent_node.agent {
			AgentRef::Same => (),
			_ => return Err("should have been same".into()),
		}

		Ok(())
	}
}

// endregion: --- Tests
