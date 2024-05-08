use crate::chain::{ChainNode, CondNode};
use serde::Deserialize;
use std::slice::Iter;

#[derive(Debug, Deserialize)]
pub struct BranchNode {
	pub branch: Vec<BranchArm>,
}

// impl iter for branch_node
impl BranchNode {
	pub fn iter(&self) -> Iter<BranchArm> {
		self.branch.iter()
	}
}

impl<'a> IntoIterator for &'a BranchNode {
	type Item = &'a BranchArm;
	type IntoIter = std::slice::Iter<'a, BranchArm>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl BranchNode {
	pub fn get_arm(&self, idx: usize) -> Option<&BranchArm> {
		self.branch.get(idx)
	}
}

#[derive(Debug, Deserialize)]
pub struct BranchArm {
	pub cond: CondNode,
	#[serde(default)] // this will make null/absent to do empty vec
	pub nodes: Vec<ChainNode>,
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use lib_utils::o_wrap;
	use serde_json::from_str;

	#[test]
	fn test_branch_node_ok() -> Result<()> {
		// -- Setup & Fixtures
		let fx_content = r#"
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
        "agent": "self"
      }]
    }
  ]
}		
    "#;

		// -- Exec
		let mut branch_node: BranchNode = from_str(fx_content)?;

		// -- Check
		let first_branch = branch_node.branch.pop().ok_or("Should have at least on e branch")?;
		let is_json = o_wrap!(first_branch.cond.input.as_ref()?.is_json).unwrap_or(false);
		assert!(is_json, "is_json should be true");

		let json_matches =
			o_wrap!(first_branch.cond.input?.json_matches?.pop()).ok_or("Should have one json_matches")?;

		assert_eq!(json_matches.pointer, "/category");
		assert_eq!(json_matches.value, 1);

		Ok(())
	}
}

// endregion: --- Tests
