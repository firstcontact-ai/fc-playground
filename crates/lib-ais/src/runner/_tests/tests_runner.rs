use crate::_test_support::{seed_all_for_test_runner, seed_mock_echo_agents};
use crate::runner::runner::run_agent_model;
use crate::runner::{resolve_stack_step, run_stack_step, RunStepStatus};
use crate::AiManager;
use lib_core::model::agent::AgentBmc;
use lib_core::model::conv::ConvBmc;
use lib_core::model::stack_step::StackStepBmc;
use lib_core::model::ModelManager;
use lib_utils::x_vec::XStringVec;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

/// Test the ChainCallStack with the message steps with the persistence in step_task
#[tokio::test]
async fn test_runner_persistent_stack_steps_walkthrough() -> Result<()> {
	// -- Setup & Fixtures
	let mm = ModelManager::new().await?;
	let aim = AiManager::default();

	let (cfile_db, conv) = seed_all_for_test_runner(&mm).await?;

	// Create the original message
	let first_input = "Hello world";
	// Note: this will create the first task_step
	let _orig_msg_id = ConvBmc::add_conv_msg(&mm, conv.id, first_input.into()).await?;

	// -- Exec
	let mut agent_names: Vec<String> = Vec::new();
	let mut model_names: Vec<String> = Vec::new();

	let mut loop_failsafe = 0;
	loop {
		// region:    --- loop_failsafe
		loop_failsafe += 1;
		if loop_failsafe > 10 {
			println!("ERROR loop_failsafe break ");
			break;
		}
		// endregion: --- loop_failsafe

		// -- Should have a next step to resolve
		let next_step_to_resolve = StackStepBmc::seek_next_to_resolve_for_conv(&cfile_db, &conv.uid)
			.await?
			.ok_or("Should have a seek_next_to_resolve")?;
		let to_resolve_step_id = next_step_to_resolve.id;
		let _resolve_status = resolve_stack_step(&mm, &cfile_db, to_resolve_step_id).await?;

		// Capture model name for the check
		let step = StackStepBmc::get(&cfile_db, to_resolve_step_id).await?;
		if let Some(model) = step.resolve_model {
			model_names.push(model);
		}

		// -- Should have next tep to run
		//    And should be the same id as the one to resolve (given the test)
		let next_step_to_run = StackStepBmc::seek_next_to_run_for_conv(&cfile_db, &conv.uid)
			.await?
			.ok_or("Should have a seek_next_to_run")?;
		let to_run_step_id = next_step_to_run.id;
		assert_eq!(
			to_run_step_id, to_resolve_step_id,
			"The two_run_step_id should have been the same as the resolve one",
		);
		let run_status = run_stack_step(&aim, &mm, &cfile_db, to_run_step_id).await?;

		// Capture agent name for the check
		let step = StackStepBmc::get(&cfile_db, to_run_step_id).await?;
		if let Some(agent_uid) = step.run_agent_uid {
			let agent = AgentBmc::get_by_uid(&mm, &agent_uid).await?;
			agent_names.push(agent.name);
		}

		match run_status {
			RunStepStatus::Ended => break,
			RunStepStatus::Ongoing => (),
		}
	} // loop

	cfile_db.print_select(
		"select id, orig_msg_id, first_step_id, prev_step_id, closer, call_stack, call_out from stack_step",
	);

	// -- Check
	assert_eq!(
		model_names.x_strs(),
		[
			"fc-mock-echo-inst",
			"fc-mock-echo-inst",
			"fc-mock-echo-inst",
			"fc-mock-echo-inst",
		]
	);
	assert_eq!(
		agent_names.x_strs(),
		["Agent One", "fc_tool_executors", "fc_tool_renderers", "Final Agent",]
	);

	Ok(())
}

#[tokio::test]
async fn test_runner_fc_mock_echo_model() -> Result<()> {
	// -- Setup & Fixtures
	let fx_inst = "agent-echo-instruction-content";
	let mm = ModelManager::new().await?;
	let aim = AiManager::default();
	let mut agents = seed_mock_echo_agents(&mm, &[("agent echo", fx_inst)]).await?;
	let agent = agents.pop().ok_or("Should have a least one agent")?;

	// -- Exec
	let res = run_agent_model(&aim, &agent, "Some input".into()).await?;

	// -- Check
	assert_eq!(res.response, fx_inst);

	Ok(())
}
