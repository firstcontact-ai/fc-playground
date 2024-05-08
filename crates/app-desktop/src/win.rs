use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::warn;

#[derive(Clone, Default)]
pub struct WinSessionManager {
	/// win_name: KeyValue
	store: Arc<Mutex<HashMap<String, WinKeyValue>>>,
}

#[derive(Default)]
pub struct WinKeyValue(HashMap<String, Value>);

impl WinSessionManager {
	pub fn get_win_value(&self, win_name: &str, key: &str) -> Value {
		let Ok(mut lock) = self.store.lock() else {
			warn!("Could not get lock, return Value::Null");
			return Value::Null;
		};

		let Some(win_kv) = lock.get_mut(win_name) else {
			return Value::Null;
		};

		win_kv.0.get(key).cloned().unwrap_or(Value::Null)
	}

	pub fn set_win_value(&self, win_name: &str, key: String, value: Value) {
		let Ok(mut lock) = self.store.lock() else {
			warn!("Could not get lock, abort");
			return;
		};

		let win_kv = lock.entry(win_name.to_string()).or_insert_with(WinKeyValue::default);
		win_kv.0.insert(key, value);
	}
}
