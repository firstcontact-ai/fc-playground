//! The `lib_rpc::response` module normalizes the JSON-RPC `.result` format for various
//! JSON-RPC APIs.
//!
//! The primary type is the simple RpcDataResponse, which contains only a `data` property.
//!
//! Note: In the future, we may introduce types like `RpcListDataResponse` that include metadata
//!       about the returned list data (e.g., pagination information).

use serde::Serialize;

#[derive(Serialize)]
pub struct RpcDataResponse<T>
where
	T: Serialize,
{
	data: T,
}

impl<T> From<T> for RpcDataResponse<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self { data: val }
	}
}
