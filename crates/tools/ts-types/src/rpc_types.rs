#![allow(unused)] // Those are just types for generation.

use lib_core::model;
use lib_rpc::{RunUserPromptParams, UserPrompt};

// NOTE: This is just the root placeholder type
//       to have one generation and avoid any possible duplication.
//       There might be a better or more elegant way to do this.
//       This type will be removed from the final .ts file
#[allow(non_camel_case_types)]
#[derive(schemars::JsonSchema)]
pub struct Types_PLACEHOLDER {
	user_prompt: UserPrompt,
	run_user_prompt_params: RunUserPromptParams,
}
