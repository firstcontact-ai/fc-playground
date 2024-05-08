use crate::event::HubEvent;
use crate::Result;
use lib_core::event::{ConvEvent, ModelEvent};
use lib_core::model::ModelManager;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Manager, Runtime};
use tracing::error;

pub fn init_plugin<R: Runtime>(mm: ModelManager) -> TauriPlugin<R> {
	// Make the plugin config optional
	// by using `Builder::<R, Option<Config>>` instead
	Builder::new("tauri-plugin-fc-event")
		.setup(move |app, _api| {
			// -- Propagate the model events
			let app_ = app.clone();
			let mm_ = mm.clone();
			tauri::async_runtime::spawn(async move {
				if let Err(err) = run_model_event_loop(app_, mm_.clone()).await {
					error!("FAIL - model_event_loop failed. Cause {err}");
				}
			});

			// -- Propagate the conv events
			let app_ = app.clone();
			let mm_ = mm;
			tauri::async_runtime::spawn(async move {
				if let Err(err) = run_conv_event_loop(app_, mm_).await {
					error!("FAIL - conv_event_loop failed. Cause {err}");
				}
			});

			Ok(())
		})
		.build()
}

// region:    --- moddel event loop

async fn run_model_event_loop<R: Runtime>(app: AppHandle<R>, mm: ModelManager) -> Result<()> {
	let mut model_subcriber = mm.hub().subscriber::<ModelEvent>()?;

	loop {
		let evt = model_subcriber.next().await?;
		if let Err(err) = exec_model_event(&app, evt).await {
			error!("ERROR while exec hubEvent for modelEvent. Cause: {err}");
		}
	}
}

async fn exec_model_event<R: Runtime>(app: &AppHandle<R>, evt: ModelEvent) -> Result<()> {
	let hub_event: HubEvent<_> = evt.into();

	// serialize to json
	if let Err(err) = app.emit("hubEvent", hub_event) {
		error!("ERROR while emitting hubEvent. Cause: {err}");
	}

	Ok(())
}

// endregion: --- moddel event loop

// region:    --- conv loop

async fn run_conv_event_loop<R: Runtime>(app: AppHandle<R>, mm: ModelManager) -> Result<()> {
	let mut conv_subscriber = mm.hub().subscriber::<ConvEvent>()?;

	loop {
		let evt = conv_subscriber.next().await?;
		if let Err(err) = exec_conv_event(&app, evt).await {
			error!("ERROR while exec hubEvent for conEvent. Cause: {err}");
		}
	}
}

pub async fn exec_conv_event<R: Runtime>(app: &AppHandle<R>, evt: ConvEvent) -> Result<()> {
	let hub_event: HubEvent<_> = evt.into();

	// serialize to json
	if let Err(err) = app.emit("hubEvent", hub_event) {
		error!("ERROR while emitting hubEvent. Cause: {err}");
	}

	Ok(())
}
// endregion: --- conv loop
