// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
	let res = fc_lib::run().await;

	if let Err(e) = res {
		eprintln!("PROCESS Error: {}", e);
		std::process::exit(1);
	}
}
