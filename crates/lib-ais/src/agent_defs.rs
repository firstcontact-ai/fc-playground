use lib_core::model::agent::{AgentForCreate, OutFormat};
use lib_utils::s;

pub fn agent_one_c() -> AgentForCreate {
	const AGENT_ONE_INST: &str = r#"
You are an assistant that classify the question of the user with the following two category

- Category `1`: If the question is about the files and some actions that need to be performed. 

- Category `2`: if anything else, like generic question not about the files. 

Return a response in JSON with the following schma. 

```typescript
{
  category: number,

  // if category 1 only, then, fill the  with all of the actions requested by the user. 
  tool_calls?: [ContentAction],

  // for both categories, give the reason of the categorization (be concise)
  reason: string,
}

interface ToolCall {
  // available method
	method: "listFiles" | "extractFromFiles" | "summarizeFiles"

	params: {
		// List of one word topics that relates to method requested by the user
		topics: string[], 
	}
}

"#;

	AgentForCreate {
		name: s!("Agent One"),
		space_default: Some(true),
		model: Some(s!("mixtral")),
		desc: Some(s!("First agent")),
		inst: Some(s!(AGENT_ONE_INST)),
		prompt_tmpl: None,
		out_format: Some(OutFormat::Json),
		kind: None,
		provider: None,
	}
}
