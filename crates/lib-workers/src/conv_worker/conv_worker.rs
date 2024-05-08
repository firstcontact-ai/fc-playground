use crate::conv_worker::Result;
use lib_ais::runner::RunStepStatus;
use lib_ais::{runner, AiManager};
use lib_core::event::{ConvEvent, Subscriber};
use lib_core::model::conv::ConvBmc;
use lib_core::model::stack_step::StackStepBmc;
use lib_core::model::ModelManager;
use tracing::{debug, error, info};

pub struct ConvWorker {
	mm: ModelManager,
	aim: AiManager,
}

impl ConvWorker {
	pub fn start(mm: ModelManager, aim: AiManager) -> Result<()> {
		let conv_worker = ConvWorker { mm, aim };

		tokio::spawn(async move {
			let res = conv_worker.start_worker().await;
			match res {
				Ok(_) => println!("ConvWorker ends OK"),
				Err(_) => println!("ConvWorker ends OK"),
			}
		});

		Ok(())
	}

	async fn start_worker(&self) -> Result<()> {
		debug!("STARTING");
		let mut sub: Subscriber<ConvEvent> = self.mm.hub().subscriber()?;

		while let Ok(evt) = sub.next().await {
			if let Err(err) = self.exec_evt(evt).await {
				error!("Evt exec fail. Cause: {err}");
			}
		}

		debug!("ENDING");

		Ok(())
	}

	async fn exec_evt(&self, evt: ConvEvent) -> Result<()> {
		debug!("EVT: {evt:?}");
		match evt {
			ConvEvent::ConvWorkNew { conv_id } => {
				debug!("ConvWorkNew for conv_id: {conv_id} - START");
				let conv = ConvBmc::get(&self.mm, conv_id).await?;
				let cfile_db = ConvBmc::getc_cfile_db(&self.mm, &conv).await?;

				if let Some(step_to_resolve) = StackStepBmc::seek_next_to_resolve_for_conv(&cfile_db, &conv.uid).await?
				{
					runner::resolve_stack_step(&self.mm, &cfile_db, step_to_resolve.id).await?;
				}

				if let Some(step_to_run) = StackStepBmc::seek_next_to_run_for_conv(&cfile_db, &conv.uid).await? {
					let run_status = runner::run_stack_step(&self.aim, &self.mm, &cfile_db, step_to_run.id).await?;
					if !matches!(run_status, RunStepStatus::Ended) {
						// we have more work to do
						ConvBmc::touch_work_tnew(&self.mm, conv_id).await?;
					} else {
						ConvBmc::touch_work_tdone(&self.mm, conv_id).await?;
					}
				}
				debug!("ConvWorkNew for conv_id: {conv_id} - END");
			}
			ConvEvent::ConvWorkDone { conv_id } => {
				let conv = ConvBmc::get(&self.mm, conv_id).await?;
				let cfile_db = ConvBmc::getc_cfile_db(&self.mm, &conv).await?;
				debug!("ConvWorkDone for conv_id: {conv_id}");

				// Check and add work if we have more to be done that have fall between the cracks
				if StackStepBmc::seek_next_to_resolve_for_conv(&cfile_db, &conv.uid)
					.await?
					.is_some()
				{
					debug!("Found other work to do, add it to the clue");
					ConvBmc::touch_work_tnew(&self.mm, conv_id).await?;
				}
			}
		}

		Ok(())
	}
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use lib_ais::_test_support::seed_all_for_test_runner;
	use lib_core::model::msg::{AuthorKind, MsgBmc};
	use lib_test_utils::sleep_ms;
	use lib_utils::trace::init_trace;
	use std::time::Duration;
	use tokio::time::sleep;

	#[tokio::test]
	async fn test_conv_work_simple() -> Result<()> {
		// -- Setup & Fixtures
		init_trace();
		let mm = ModelManager::new().await?;
		let aim = AiManager::default();
		let (cfile_db, conv) = seed_all_for_test_runner(&mm).await?;
		ConvWorker::start(mm.clone(), aim.clone())?;

		// Note: Somehow we have to pause here, otherwise, the events are not received.
		//       We should investigate the reason.
		//       In prod code, not an issue, as the queues are started at start of the app.
		//       So there are an normal pause there.
		sleep_ms(1).await;

		// -- Exec
		// Create the original message
		let first_input = "Hello world";
		// Note: this will create the first task_step
		let _orig_msg_id = ConvBmc::add_conv_msg(&mm, conv.id, first_input.into()).await?;

		// we pause to let the worker work
		sleep_ms(10).await;

		// for debug
		cfile_db.print_table("msg")?;
		cfile_db.print_select(
			"select id, uid, orig_msg_id, resolve_model, run_agent_uid, first_step_id, prev_step_id, closer, call_stack from stack_step",
		)?;

		// -- Check the Msgs
		let mut msgs = MsgBmc::list(&cfile_db, None, None).await?.into_iter();
		let msg_input = msgs.next().ok_or("Should have msg_input")?;
		let msg_output = msgs.next().ok_or("Should have msg_output")?;
		// check input
		assert_eq!(msg_input.content.ok_or("should have content")?, "Hello world");
		assert!(matches!(
			msg_input.author_kind.ok_or("should have author kind")?,
			AuthorKind::User
		));
		assert_eq!(msg_input.orig_msg_id, None);
		// check output
		assert_eq!(msg_output.content.ok_or("should have content")?, "Final Agent response");
		assert!(matches!(
			msg_output.author_kind.ok_or("should have author kind")?,
			AuthorKind::Agent
		));
		assert_eq!(msg_output.orig_msg_id, Some(msg_input.id));

		// -- Check the StackStep
		let mut steps = StackStepBmc::list(&cfile_db, None, None).await?;
		assert_eq!(steps.len(), 5, "Should have 5 StackSteps");

		Ok(())
	}
}

// endregion: --- Tests
