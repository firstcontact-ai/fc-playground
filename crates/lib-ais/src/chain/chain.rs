use crate::chain::{AgentNode, BranchArm, ChainNode, InputContent};
use crate::{Error, Result};
use lib_core::model::agent::Agent;
use serde::{Deserialize, Serialize};

// region:    --- Chain CallStack/Cursor

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainCallStack {
	pub items: Vec<StackItem>,
}

impl ChainCallStack {
	pub fn new_at_agent(uid: impl Into<String>) -> Self {
		Self {
			items: vec![StackItem::new_at_start(uid)],
		}
	}

	pub fn last_item(&self) -> Option<&StackItem> {
		self.items.last()
	}

	pub fn pop_item(&mut self) -> Option<StackItem> {
		self.items.pop()
	}

	pub fn push_item(&mut self, item: StackItem) {
		self.items.push(item);
	}

	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}

	pub fn to_json(&self) -> Result<String> {
		serde_json::to_string(self).map_err(Error::FailSerializeStack)
	}

	// from_json
	pub fn from_json(json_str: &str) -> Result<Self> {
		serde_json::from_str(json_str).map_err(Error::StackFailParse)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackItem {
	pub agent_uid: String,
	pub cursor: ChainCursor,
}

impl StackItem {
	pub fn new_at_start(uid: impl Into<String>) -> Self {
		Self {
			agent_uid: uid.into(),
			cursor: ChainCursor::default(),
		}
	}
	pub fn new(uuid: impl Into<String>, cursor: ChainCursor) -> Self {
		Self {
			agent_uid: uuid.into(),
			cursor,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChainCursor {
	pub idxs: Vec<usize>,
}

// endregion: --- Chain Stack/Cursor

#[derive(Debug, Deserialize)]
pub struct Chain {
	pub nodes: Vec<ChainNode>,
}

/// impl default for Chain (one ChainNode)
impl Default for Chain {
	fn default() -> Self {
		Self {
			nodes: vec![AgentNode::default().into()],
		}
	}
}

impl Chain {
	pub fn new(chain_str: Option<&str>) -> Result<Chain> {
		let chain = match chain_str.filter(|c| !c.trim().is_empty()) {
			Some(chain_str) => serde_json::from_str(chain_str).map_err(Error::ChainFailParse)?,
			None => Chain::default(),
		};

		Ok(chain)
	}

	#[allow(clippy::assigning_clones)]
	pub fn get_el<'a>(&'a self, idxs: &[usize]) -> Option<ChainEl<'a>> {
		// -- Initialize the iterator context
		let mut f_els = Some(ChainEl::Chain(self));
		let mut f_el: Option<ChainEl> = None;

		// -- Iterate and go down each time
		for &idx in idxs {
			f_el = f_els?.get_el(idx);
			// TODO: remove clippy::assigning_clones Need to clean/optimize this below
			f_els = f_el.clone();
		}

		// -- Return the final options
		f_el
	}

	/// Get the pointed AgentNode for a given cursor.
	/// - Will return None if no agent node at this cursor.
	/// - Do not do any cursor/idxs modification.
	pub fn get_agent_node(&self, cursor: &ChainCursor) -> Option<&AgentNode> {
		let el = self.get_el(&cursor.idxs)?;
		match el {
			ChainEl::Node(ChainNode::Agent(agent_node)) => Some(agent_node),
			_ => None,
		}
	}

	/// Get the ChainCursor for the next AgentNode
	/// - Only turn a ChainCursor if there is a next AgentNode for the given input
	/// - Will evaluation the input content when getting into a branch node.
	pub fn next_agent_cursor(&self, cursor: &ChainCursor, input: &InputContent) -> Option<ChainCursor> {
		// -- Extract vals from cursor
		// returns early if no idxs
		let mut idxs = cursor.idxs.clone();

		let mut query_next = true;

		loop {
			// we get the next node_idxs
			idxs = if query_next {
				self.next_node_idxs(&idxs)?
			} else {
				query_next = true;
				idxs
			};

			let el = self.get_el(&idxs)?;

			match el {
				// if the pointed el is an agent,
				ChainEl::Node(ChainNode::Agent(_agent_node)) => break,

				ChainEl::Node(ChainNode::Branch(branch_node)) => {
					// FIXME: Needs to evaluate input and cond of arm
					for (arm_idx, arm) in branch_node.iter().enumerate() {
						if arm.cond.matches_input(input) {
							idxs.push(arm_idx); // we go into the arm idx
							idxs.push(0); // point to the first eventual item (if none, it will exit the branch)
							query_next = false;
						}
					}
				}

				// Note: Should not reach this point, and the next_node_idxs will not drill down into branch arms
				ChainEl::Arm(_) => {
					println!("->> ChainEl::Arm SHOULD NOT BE REACHED");
					return None;
				}

				// Note: Should not reach this point, as this is past the starting point.
				//       Chain can nest agents that thats Chain, but cannot nest chain directly.
				ChainEl::Chain(_) => return None,
			}
		}

		Some(ChainCursor { idxs })
	}

	/// Get the next node idxs for a given `idxs`
	/// Notes:
	/// - Will only return idxs for BranchNode and AgentNode
	/// - Will not go down the Branch Arms
	/// - But if idxs of a arm node, then will go to next node of the same arm
	///   or back up to the node after the containing chain.
	/// - Conseqently, the first node of a agent Node need to be resolved by the caller
	///   as this function will always go +1 on the arm nodes idx.
	///
	fn next_node_idxs(&self, idxs: &[usize]) -> Option<Vec<usize>> {
		// -- If empty test the first node
		if idxs.is_empty() {
			let idxs = vec![0];
			return self.get_el(&idxs).map(|_| idxs);
		}

		// Note: This will loop and update the indexes to find indexes for the next element.
		//       It will increment and pop the last index as needed.
		//       This design allows avoiding recursive calls.
		let mut idxs = idxs.to_owned();

		loop {
			// When empty, it means we ended the whole walkthrough
			// so, returning None
			if idxs.is_empty() {
				return None;
			}

			// Get the element for these indexes,
			// and if none exists, remove this index and increase the parent
			// so that the next iteration of this loop can look at the next item.
			// Note: This will eventually revert to empty idxs, which will return None.
			let Some(el) = self.get_el(&idxs) else {
				pop_and_inc(&mut idxs);

				// if this is a Agent node, then, returns it (otherwise match below will go to next)
				if let Some(ChainEl::Node(ChainNode::Agent(_))) = self.get_el(&idxs) {
					return Some(idxs);
				}
				continue;
			};

			match el {
				// If we have an AgentNode or BranchNode, then, we inc last
				// and return if we have something
				// (otherwise, next iteration will go back to parent increment)
				ChainEl::Node(_) => {
					inc_last(&mut idxs);
					// if we have a match for the next agent, then, we can return early
					if self.get_el(&idxs).is_some() {
						return Some(idxs);
					}
				}

				// When we are in arm, by design we go back to the branch
				// so that the next will be what even el is after the branch
				// Note 1: That's mean that the caller, need to find the arm and then, add the `0` idx
				//         to navigate down the arm nodes.
				// Note 2: This is because the Branch needs to run one arm only, so, it's not a simple next.
				ChainEl::Arm(_arm) => {
					idxs.pop(); // go back to branch
				}

				// Chain should not be possible, so, return None
				ChainEl::Chain(_) => return None,
			};
		} // loop
	}

	/// A simple step through idxs for each el.
	/// Will get down branch arm.
	/// NOT USED YET - might not be needed
	#[allow(unused)]
	pub fn next_el_idxs(&self, idxs: &[usize]) -> Option<Vec<usize>> {
		// -- If empty test the first node
		if idxs.is_empty() {
			let idxs = vec![0];
			return self.get_el(&idxs).map(|_| idxs);
		}

		// Note: This will loop and update the indexes to find indexes for the next element.
		//       It will increment and pop the last index as needed.
		//       This design allows avoiding recursive calls.
		let mut idxs = idxs.to_owned();

		loop {
			// When empty, it means we ended the whole walkthrough
			// so, returning None
			if idxs.is_empty() {
				return None;
			}

			// Get the element for these indexes,
			// and if none exists, remove this index and increase the parent
			// so that the next iteration of this loop can look at the next item.
			// Note: This will eventually revert to empty idxs, which will return None.
			let Some(el) = self.get_el(&idxs) else {
				pop_and_inc(&mut idxs);
				continue;
			};

			match el {
				// In both Agent Node or Arm, we increment the last index.
				// - If some element, then we can return the new idxs.
				// - If none, the next iteration will take care of performing the pop_and_inc,
				//   so that the subsequent iteration can check the next parent element.
				ChainEl::Node(ChainNode::Agent(_)) | ChainEl::Arm(_) => {
					inc_last(&mut idxs);
					// if we have a match for the next agent, then, we can return early
					if self.get_el(&idxs).is_some() {
						return Some(idxs);
					}
				}

				// if we have a branch, and we have a arm at 0, then,
				// we add `0` to the idxs so that this can be taken in acccount
				ChainEl::Node(ChainNode::Branch(branch)) => {
					// if we have an arm at 0, then, we can return early.
					if branch.get_arm(0).is_some() {
						idxs.push(0);
						return Some(idxs);
					}
					// otherwise, empty branch, and so we can pop and inc
					pop_and_inc(&mut idxs);
				}

				// Chain should not be possible, so, return None
				ChainEl::Chain(_) => return None,
			};
		}

		// -- Fallback
		None
	}
}

fn inc_last(idxs: &mut [usize]) {
	if let Some(last) = idxs.last_mut() {
		*last += 1;
	}
}

fn pop_and_inc(idxs: &mut Vec<usize>) {
	idxs.pop();
	if let Some(last) = idxs.last_mut() {
		*last += 1;
	}
}

#[derive(Clone, Debug)]
pub enum ChainEl<'a> {
	Chain(&'a Chain),
	Node(&'a ChainNode),
	Arm(&'a BranchArm),
}

impl<'a> ChainEl<'a> {
	fn get_el(&self, idx: usize) -> Option<ChainEl<'a>> {
		match self {
			ChainEl::Chain(chain) => chain.nodes.get(idx).map(ChainEl::Node),
			ChainEl::Node(ChainNode::Agent(_)) => None,
			ChainEl::Node(ChainNode::Branch(branch)) => branch.branch.get(idx).map(ChainEl::Arm),
			ChainEl::Arm(arm) => arm.nodes.get(idx).map(ChainEl::Node),
		}
	}
}

// region:    --- Model Agent to Chain Trait

pub trait AgentChain {
	fn get_chain(&self) -> Result<Chain>;
}

impl AgentChain for Agent {
	fn get_chain(&self) -> Result<Chain> {
		let chain_str = self.chain.as_deref();
		Chain::new(chain_str)
	}
}

// endregion: --- Model Agent to Chain Trait

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_test_support::mock_name_from_agent_node;
	use lib_utils::x_vec::XStringVec;
	use serde_json::from_str;

	const CHAIN: &str = r#"
{
	"nodes": [
		{
			"agent": {"name": "Some-Agent"},
			"name_input": "original_input"
		}, 
		{
			"branch": [
				{ 
					"cond": {
						"input": {
							"is_json": true,
							"json_matches": {
								"pointer": "/category",
								"value": 1
							}
						}
					},
					"nodes": [{
						"agent": { "name": "fc_tool_executors" }
					}, {
						"agent": { "name": "fc_tool_renderers" }
					}]			
				},
				{
					"cond": {
						"input": {
							"is_json": true,
							"json_matches": {
								"pointer": "/category",
								"value": 2
							}
						}
					},
					"nodes": [{
						"agent": { 
							"uid": "Generic Agent", 
							"input": "${original_input}"
						}
					}]					
				}			
			]
		},
		{
			"agent": {"name": "Final Agent"}
		}		
	]
}
"#;

	#[test]
	fn test_chain_get_el() -> Result<()> {
		// -- Setup & Fixtures
		let fx_chain: Chain = from_str(CHAIN)?;

		// -- Exec/Check node [0]
		let el_0 = fx_chain.get_el(&[0]).ok_or("should have node [0]")?;
		assert!(matches!(el_0, ChainEl::Node(ChainNode::Agent(_))));

		// -- Exec/Check node [0, 1] - should be None
		let el_0_1 = fx_chain.get_el(&[0, 1]);
		assert!(el_0_1.is_none());

		// -- Exec/Check node [1]
		let el_1 = fx_chain.get_el(&[1]).ok_or("should have node [1]")?;
		assert!(matches!(el_1, ChainEl::Node(ChainNode::Branch(_))));

		// -- Exec/Check node [1, 0]
		let el_1_0 = fx_chain.get_el(&[1, 0]).ok_or("should have node [1, 0]")?;
		assert!(matches!(el_1_0, ChainEl::Arm(_)));

		// -- Exec/Check node [1, 0, 0] - should be agent noe
		let el_1_0_0 = fx_chain.get_el(&[1, 0, 0]).ok_or("should have node [1, 0, 0]")?;
		assert!(matches!(el_1_0_0, ChainEl::Node(ChainNode::Agent(_))));

		// println!("->!!>!! {el_0:?}");

		Ok(())
	}

	#[test]
	fn test_chain_next_agent_node() -> Result<()> {
		// -- Setup & Fixtures
		let fx_chain: Chain = from_str(CHAIN)?;
		let fx_input_contents: &[InputContent] =
			&[InputContent::new(r#"{"category": 1}"#), InputContent::new(r#"{"category": 2}"#)];
		let fx_names: &[&[&str]] = &[
			//
			&["Some-Agent", "fc_tool_executors", "fc_tool_renderers", "Final Agent"],
			&["Some-Agent", "Generic Agent", "Final Agent"],
		];

		// -- Execs & Checks
		for (input_content, &fx_agent_names) in fx_input_contents.iter().zip(fx_names.iter()) {
			let mut failsafe_count = 0;
			let mut agent_names: Vec<String> = Vec::new();

			let mut cursor = Some(ChainCursor::default());
			loop {
				cursor = cursor.and_then(|c| fx_chain.next_agent_cursor(&c, input_content));

				// if no cursor, we exit the loop
				let Some(cursor_ref) = cursor.as_ref() else {
					break;
				};

				// if cursor, we capture the agent and its mock_name
				let Some(agent) = fx_chain.get_agent_node(cursor_ref) else {
					return Err(format!("No AgentNode found for cursor {cursor_ref:?}").into());
				};

				let name = mock_name_from_agent_node(agent);
				agent_names.push(name);

				// handling infinite loop failsafe
				failsafe_count += 1;
				if failsafe_count > 10 {
					return Err("Seems to be an infinite loop.".into());
				}
			}

			// -- Checks
			assert_eq!(agent_names.x_strs(), fx_agent_names);
		}

		Ok(())
	}
}

// endregion: --- Tests
