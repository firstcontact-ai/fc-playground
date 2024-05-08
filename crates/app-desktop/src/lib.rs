// region:    --- Modules

// -- Privates
mod error;
mod event;
mod init;
mod ipc;
mod win;

// -- Flatten
pub use self::error::{Error, Result};

// -- Imports
use crate::init::init_main_db_all;
use crate::win::WinSessionManager;
use lib_core::model::ModelManager;
use lib_rpc::all_rpc_router_buider;
use lib_workers::conv_worker::ConvWorker;
use lib_workers::dsource_worker::DSourceWorker;

// endregion: --- Modules

const TRACE_FILTER: &str = r#"
app_desktop=debug,
lib_ais=debug,
lib_core=debug,
lib_rpc=debug,
lib_utils=debug,
lib_workers=debug
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
	// -- Setup tracing
	lib_utils::trace::init_trace();

	// -- Setup ModelManager
	let mm = ModelManager::new().await?;

	// -- Init the AI Manager
	let aim = lib_ais::AiManager::default();

	// -- Init the Dbs
	init_main_db_all(&mm).await?;

	// -- Init the workers
	DSourceWorker::start(mm.clone())?;
	ConvWorker::start(mm.clone(), aim.clone())?;

	// -- Setup RPC States
	let rpc_router = all_rpc_router_buider().append_resource(mm.clone()).append_resource(aim).build();

	// -- Start Tauri App
	let app_ctx = tauri::generate_context!();
	tauri::Builder::default()
		.plugin(tauri_plugin_shell::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(event::init_plugin(mm))
		.manage(rpc_router)
		.manage(WinSessionManager::default())
		.invoke_handler(tauri::generate_handler![
			// -- Rpc
			ipc::rpc,
			// -- Win
			ipc::get_win_size,
			ipc::get_win_sess_value,
			ipc::set_win_sess_value,
		])
		.run(app_ctx)
		.expect("error while running tauri application");

	Ok(())
}
