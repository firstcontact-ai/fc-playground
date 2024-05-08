use crate::chain::{AgentNode, BranchNode};
use core::fmt;
use derive_more::From;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{from_value, Value};

#[derive(Debug, From)]
pub enum ChainNode {
	Agent(AgentNode),
	Branch(BranchNode),
}

// region:    --- Deserializer

impl<'de> Deserialize<'de> for ChainNode {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct FlowNodeVisitor;

		impl<'de> Visitor<'de> for FlowNodeVisitor {
			type Value = ChainNode;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("an object with either an 'agent' or a 'branch' key")
			}

			fn visit_map<V>(self, mut map: V) -> Result<ChainNode, V::Error>
			where
				V: MapAccess<'de>,
			{
				let value = Value::deserialize(de::value::MapAccessDeserializer::new(&mut map))?;
				let flow_node = if value.get("agent").is_some() {
					ChainNode::Agent(from_value(value).map_err(de::Error::custom)?)
				} else if value.get("branch").is_some() {
					ChainNode::Branch(from_value(value).map_err(de::Error::custom)?)
				} else {
					return Err(de::Error::custom("Not a value for chain node (no agent or branch)"));
				};

				Ok(flow_node)
			}
		}

		deserializer.deserialize_map(FlowNodeVisitor)
	}
}

// endregion: --- Deserializer
