// region:    --- Modules

mod model_types;
mod rpc_types;

use schemars::{schema_for, JsonSchema};
use simple_fs::{ensure_dir, watch};
use std::fs;
use std::process::{Command, Stdio};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

// endregion: --- Modules

const SCHEMA_BASE_PATH: &str = "frontend/dist-ui/js/bindings";
const TS_TYPE_BASE_PATH: &str = "frontend/src-ui/src/bindings";

// const WATCH_PATHS: &[&str] = &["crates/"];

fn main() -> Result<()> {
	ensure_dir(SCHEMA_BASE_PATH)?;
	ensure_dir(TS_TYPE_BASE_PATH)?;

	gen_all_types()?;

	let watch_mode = std::env::args().any(|s| s == "-w");
	if watch_mode {
		let watcher = watch("crates/")?;
		loop {
			match watcher.rx.recv() {
				Ok(evts) => {
					if evts.iter().any(|e| e.spath.to_str().ends_with(".rs")) {
						println!("ts-types -  .rs changed detected");
						gen_all_types()?;
					}
				}
				Err(err) => {
					println!("ts-types - end of watch loop {err:?}");
					break;
				}
			}
		}

		println!("The end of the ts-types watch");
	}

	Ok(())
}

fn gen_all_types() -> Result<()> {
	gen_ts_types::<model_types::Types_PLACEHOLDER>("model_types")?;
	gen_ts_types::<rpc_types::Types_PLACEHOLDER>("rpc_types")?;

	Ok(())
}

fn gen_ts_types<T: JsonSchema>(name: &str) -> Result<()> {
	let schema_path = format!("{SCHEMA_BASE_PATH}/{name}.json");
	let ts_path = format!("{TS_TYPE_BASE_PATH}/{name}.ts");

	// -- Generate json schema
	let root_schema = schema_for!(T);
	let schema_content = serde_json::to_string_pretty(&root_schema)?;
	// Save json schema file
	fs::write(&schema_path, schema_content)?;
	println!("Generated - {schema_path}");

	// -- Call scripts/src/ts-types to generate typescript types
	let args = &["run", "scripts/src/gen-ts-types.ts", &schema_path, &ts_path];
	exec_cmd("bun", args, Some("frontend"))?;

	Ok(())
}

fn exec_cmd(cmd: &str, args: &[&str], working_dir: Option<&str>) -> Result<()> {
	let mut cmd = Command::new(cmd);
	cmd.args(args);
	if let Some(working_dir) = working_dir {
		cmd.current_dir(working_dir);
	}
	cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
	let output = cmd.output()?;

	if !output.status.success() {
		return Err("Failed to run the command: 'bun ...'".into());
	}
	Ok(())
}
