use crate::chain::{resolve_agent, AgentChain, ChainCallStack, InputContent, StackItem};
use crate::{AiManager, GenReq, GenRes};
use crate::{Error, Result};
use lib_core::model::agent::{Agent, AgentBmc};
use lib_core::model::conv::ConvBmc;
use lib_core::model::msg::MsgBmc;
use lib_core::model::stack_step::{StackStep, StackStepBmc, StackStepForUpdate};
use lib_core::model::support::prelude::SlDb;
use lib_core::model::{Id, ModelManager};
use lib_utils::x_string::XStr as _;
use lib_utils::{hbs, s};
use serde_json::json;
use tracing::{error, warn};

/// The `resolve_stack_step`
/// - `CloserStep` means that the step has the flag `is_closer` and stack is empty.
/// - `RunningStep` means the step is a running stak `is_closer == 0`
///    and stack has something that needs to be ran.
pub enum ResolveState {
	CloserStep,
	RunningStep,
}

/// Resolve and save the new `CallStack` for this StackStep
/// This will:
/// - It get the prev_stack from the prev step
/// - Get the input from the prev_stack output or orginal msg input
/// - Resolve compute and save the next stack from the prev_stack
/// - If the stack is empty, it will update the step as the "closer" step for this msg
/// - Return the ResolveState telling if there is a stack or not.
pub async fn resolve_stack_step(mm: &ModelManager, cfile_db: &SlDb, step_id: Id) -> Result<ResolveState> {
	StackStepBmc::set_resolve_tstart(cfile_db, step_id).await?;

	let step = StackStepBmc::get(cfile_db, step_id).await?;

	// -- Resolve the prev stack
	// if we have a prev_step_id, then, take the stack from there.
	let (prev_stack, prev_output) = get_prev_stack_and_output(mm, cfile_db, &step).await?;

	// let _prev_stack_str = format!("{prev_stack:?}");
	// -- Resolve the current stack
	//    (which is the next stack of the prev_stack)
	let input = InputContent::new(prev_output);
	let computed_stack = compute_next_stack(mm, prev_stack, input).await?;
	let resolve_model = if let Some(agent_uid) = computed_stack.last_item().map(|si| si.agent_uid.as_str()) {
		let agent = AgentBmc::get_by_uid(mm, agent_uid).await?;
		agent.model
	} else {
		None
	};

	let state = if computed_stack.is_empty() {
		ResolveState::CloserStep
	} else {
		ResolveState::RunningStep
	};

	// update in the db
	StackStepBmc::update_resolve_success(
		cfile_db,
		step_id,
		StackStepForUpdate {
			resolve_model,
			call_stack: Some(computed_stack.to_json()?),
			..Default::default()
		},
		matches!(state, ResolveState::CloserStep),
	)
	.await?;

	Ok(state)
}

/// The run_stack_step response status.
/// NOTE: variant should have better names
#[derive(Debug)]
pub enum RunStepStatus {
	Ended,
	Ongoing,
}

/// Run a stack_step for the model at the step call stack location
/// - IMPORTANT: This assumes the `resolve_stack_step` was already called on the step
/// - Branch
///   - if step is a closer, then,
///   - if step is not a closer, then, run the agent model, and create next step
pub async fn run_stack_step(aim: &AiManager, mm: &ModelManager, cfile_db: &SlDb, step_id: Id) -> Result<RunStepStatus> {
	StackStepBmc::set_run_tstart(cfile_db, step_id).await?;
	let step = StackStepBmc::get(cfile_db, step_id).await?;

	// If closer, create the answer Msg
	let state = if step.closer {
		let prev_out = StackStepBmc::get_prev_step_call_out(cfile_db, step_id).await?;
		let prev_out = prev_out.ok_or(Error::PrevStepHasNoOuput(step_id))?;

		MsgBmc::create_agent_answer(cfile_db, step, prev_out).await?;
		RunStepStatus::Ended
	}
	// if not a closer, run agent, and create next step
	else {
		match run_stack_step_agent(aim, mm, cfile_db, &step).await {
			// -- if we have run success
			Ok((agent, res)) => {
				// update the step with success
				StackStepBmc::update_run_end_success(
					cfile_db,
					step_id,
					StackStepForUpdate {
						call_out: Some(res.response),
						run_agent_uid: Some(agent.uid),
						run_agent_name: Some(agent.name),
						..Default::default()
					},
				)
				.await?;

				// create the next step
				StackStepBmc::create_next_from_step(cfile_db, step_id).await?;

				// return ongon
				RunStepStatus::Ongoing
			}
			Err(err) => {
				error!("Fail to run step {step_id} cause: {err}");
				StackStepBmc::update_run_end_fail(
					cfile_db,
					step_id,
					StackStepForUpdate {
						call_err: Some(err.to_string()),
						..Default::default()
					},
				)
				.await?;

				return Err(err);
			}
		}
	};
	Ok(state)
}

pub async fn run_stack_step_agent(
	aim: &AiManager,
	mm: &ModelManager,
	cfile_db: &SlDb,
	step: &StackStep,
) -> Result<(Agent, GenRes)> {
	let step_id = step.id;

	// -- get the call stack
	let mut stack = ChainCallStack::from_json(step.call_stack.as_ref().ok_or(Error::StackStepNotFound(step_id))?)?;

	let Some(sitem) = stack.pop_item() else {
		// TODO: Might need to handle differently (return error)
		warn!("run_stack_step_agent - nothing to be ran, no stack item in stack - abort, skip");
		return Err(Error::CantRunStepStackEmpty { step_id: step.id });
	};

	// -- Resolve
	let agent_uid = sitem.agent_uid;
	let (_, prev_output) = get_prev_stack_and_output(mm, cfile_db, step).await?;
	let input = InputContent::new(prev_output);

	let agent = AgentBmc::get_by_uid(mm, &agent_uid).await?;
	let res = run_agent_model(aim, &agent, input).await?;

	Ok((agent, res))
}

/// Run the agent model for a given input
/// TODO: needs to remove pub
async fn run_agent_model(aim: &AiManager, agent: &Agent, input: InputContent) -> Result<GenRes> {
	// -- Get the model
	let model = agent.model.as_ref().ok_or_else(|| Error::AgentHasNoModel {
		agent_id: agent.id,
		agent_name: agent.name.to_string(),
	})?;

	// -- Get the ai client for the agent model
	let ai_model = agent.model.as_deref().unwrap_or_default();
	let ai_client = aim.get_client_for_model(ai_model).await?;

	// -- Resolve the agent prompt
	let prompt = render_prompt_tmpl(agent, input)?;

	// -- Exec the GenReq
	let gen_req = GenReq {
		prompt,
		inst: agent.inst.clone(),
		out_format: agent.out_format.clone(),
	};
	let gen_res = ai_client.gen(model, gen_req).await?;

	Ok(gen_res)
}

fn render_prompt_tmpl(agent: &Agent, input: InputContent) -> Result<String> {
	if let Some(prompt_tmpl) = agent.prompt_tmpl.x_non_empty_str() {
		let data = match input {
			InputContent::Text(text) => json!({"input": text}),
			InputContent::Json(value) => json!({"input": value}),
		};
		let res = hbs::render(prompt_tmpl, data).map_err(Error::FailToHbsRenderPrompt)?;
		Ok(res)
	} else {
		Ok(input.to_string())
	}
}

// region:    --- Support

async fn compute_next_stack(
	mm: &ModelManager,
	mut stack: ChainCallStack,
	input: InputContent,
) -> Result<ChainCallStack> {
	let mut loop_failsafe = 0;
	loop {
		// -- failsafe_count
		loop_failsafe += 1;
		if loop_failsafe > 10 {
			warn!("compute_next_stack loop over 10 - stop looop");
			break;
		}

		// -- Pop the last item
		let Some(sitem) = stack.pop_item() else {
			return Ok(stack);
		};

		// -- Get the chain, base_agent, cursor
		let agent_uid = sitem.agent_uid;
		let cursor = sitem.cursor;
		let agent = AgentBmc::get_by_uid(mm, &agent_uid).await?;
		let chain = agent.get_chain()?;

		let next_agent_cursor = chain.next_agent_cursor(&cursor, &input);

		if let Some(next_agent_cursor) = next_agent_cursor {
			let next_agent_node = chain
				.get_agent_node(&next_agent_cursor)
				.expect("FATAL no AGENT NODE FOR CURSOR");
			let next_agent = resolve_agent(mm, agent.id, next_agent_node).await?;

			if next_agent.id == agent.id {
				stack.push_item(StackItem::new(&next_agent.uid, next_agent_cursor));
				// we return early
				return Ok(stack);
			} else {
				// we first add the nexxt agent cursor for this input
				if let Some(next_cursor) = chain.next_agent_cursor(&cursor, &input) {
					stack.push_item(StackItem::new(&agent_uid, next_cursor))
				}
				// Then, we add a new stack item for the new agent (init, empty cursor)
				stack.push_item(StackItem::new_at_start(&next_agent.uid))
			}
		}
	}

	// TODO FIXME
	Ok(stack)
}

/// will eget the stack from prev step or build a new one from the msg orig
async fn get_prev_stack_and_output(
	mm: &ModelManager,
	cfile_db: &SlDb,
	step: &StackStep,
) -> Result<(ChainCallStack, String)> {
	// if we have a prev step, get it from prev_step
	let (prev_stack, prev_output) = if let Some(prev_step_id) = step.prev_step_id {
		let prev_step = StackStepBmc::get(cfile_db, prev_step_id).await?;
		let stack = prev_step.call_stack.ok_or(Error::PrevStepHasNoStack(prev_step_id))?;
		let stack = ChainCallStack::from_json(&stack)?;
		let output = prev_step.call_out.ok_or(Error::PrevStepHasNoOuput(prev_step_id))?;
		(stack, output)
	}
	// otherwise, get it from msg orig
	else {
		let msg = MsgBmc::get(cfile_db, step.orig_msg_id).await?;
		let output = msg.content.ok_or(Error::StepMsgHasNoContent(step.id))?;
		// get agent_uuid for msg
		let agent_uid = ConvBmc::get_agent_uid_for_msg_id(mm, cfile_db, msg.id).await?;
		let stack = ChainCallStack::new_at_agent(agent_uid);
		(stack, output)
	};

	Ok((prev_stack, prev_output))
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "_tests/tests_runner.rs"]
mod tests;

// endregion: --- Tests
