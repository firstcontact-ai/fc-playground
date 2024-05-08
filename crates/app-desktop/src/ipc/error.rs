use derive_more::From;
use serde::Serialize;
use serde_json::Value;
use tracing::warn;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Serialize)]
pub enum Error {
	#[from]
	Custom(String),

	// -- App Libs
	#[from]
	LibRpc(lib_rpc::Error),

	// -- Rpc
	#[from]
	RpcRequestParsing(rpc_router::RequestParsingError),

	// When encountering `rpc_router::Error::Handler`, we deconstruct it into the appropriate concrete application error types.
	RpcLibRpc(lib_rpc::Error),
	// ... more types might be here, depending on our Error strategy. Usually, one per library crate is sufficient.

	// When it's `rpc_router::Error::Handler` but we did not handle the type,
	// we still capture the type name for information. This should not occur once the code is complete.
	RpcHandlerErrorUnhandled(&'static str),

	// When the `rpc_router::Error` is not a `Handler`, we can pass through the rpc_router::Error
	// as all variants contain concrete types.
	RpcRouter {
		id: Value,
		method: String,
		error: rpc_router::Error,
	},
}

// region:    --- Froms

// region:    --- From rpc-router::Error

/// The purpose of this `From` implementation is to extract the error types we recognize
/// from the `rpc_router`'s `RpcHandlerError` within the `rpc_router::Error::Handler`
/// and place them into the appropriate variant of our application error enum.
///
/// - The `rpc-router` provides an `RpcHandlerError` scheme to allow application RPC handlers
/// to return the errors they wish with minimal constraints.
/// - This approach requires us to "unpack" those types in our code and assign them to the correct
/// "concrete/direct" variant (not `Box<dyn Any>`...).
/// - If it's not an `rpc_router::Error::Handler` variant, then we can capture the `rpc_router::Error`
/// as it is, treating all other variants as "concrete/direct" types.
impl From<rpc_router::CallError> for Error {
	fn from(rpc_call_error: rpc_router::CallError) -> Self {
		let rpc_router::CallError {
			id,
			method,
			error: rpc_router_error,
		} = rpc_call_error;

		match rpc_router_error {
			rpc_router::Error::Handler(mut rpc_handler_error) => {
				if let Some(lib_rpc_error) = rpc_handler_error.remove::<lib_rpc::Error>() {
					Error::RpcLibRpc(lib_rpc_error)
				}
				// report the unhandled error for debugging and completing code.
				else {
					let type_name = rpc_handler_error.type_name();
					warn!("Unhandled RpcHandlerError type: {type_name}");
					Error::RpcHandlerErrorUnhandled(type_name)
				}
			}
			error => Error::RpcRouter { id, method, error },
		}
	}
}

// endregion: --- From rpc-router::Error

impl From<&str> for Error {
	fn from(val: &str) -> Self {
		Self::Custom(val.to_string())
	}
}

// endregion: --- Froms

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
