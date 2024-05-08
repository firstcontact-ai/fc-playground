// region:    --- Modules

// -- Privates
mod error;

// -- Flatten
pub use self::error::Result;

// -- Imports
use crate::win::WinSessionManager;
use rpc_router::{CallResponse, Router};
use serde_json::{json, Value};
use tauri::{State, Window};

// endregion: --- Modules

// region:    --- IPC Rpc

#[tauri::command]
pub async fn rpc(rpc_router: State<'_, Router>, rpc_request: Value) -> Result<Value> {
	let rpc_request = rpc_router::Request::from_value(rpc_request)?;

	let rpc_response = rpc_router.call(rpc_request).await?;
	let CallResponse { id, value, .. } = rpc_response;

	Ok(json!({
		"id": id, // for now just harcode to 1. TODO: needs to take from call
		"result": value
	}))
}

// endregion: --- IPC Rpc

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn get_win_sess_value(
	win: Window,
	win_sess: State<'_, WinSessionManager>,
	key: String,
) -> tauri::Result<Value> {
	let val = win_sess.get_win_value(win.label(), &key);
	Ok(val)
}

#[tauri::command]
pub async fn get_win_size(win: Window) -> tauri::Result<(u32, u32)> {
	let size = win.outer_size().unwrap();
	Ok((size.width, size.height))
}

#[tauri::command]
pub async fn set_win_sess_value(
	win: Window,
	win_sess: State<'_, WinSessionManager>,
	key: String,
	value: Value,
) -> tauri::Result<()> {
	win_sess.set_win_value(win.label(), key, value);
	Ok(())
}
