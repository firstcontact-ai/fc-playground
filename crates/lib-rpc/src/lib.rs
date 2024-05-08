// region:    --- Modules

// -- Privates
mod error;
mod response;
mod rpc_params;
mod rpc_result;
mod rpcs;
mod types;

// -- Flatten
pub use error::{Error, Result};
pub use rpc_params::*;
pub use rpcs::all_rpc_router_buider;
pub use types::*;

// endregion: --- Modules
