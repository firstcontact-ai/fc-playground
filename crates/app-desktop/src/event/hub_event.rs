use lib_core::event::{ConvEvent, ModelEvent};
use lib_core::model::EntityRef;
use lib_utils::s;
use lib_utils::x_value::XValue;
use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct HubEvent<D> {
	pub hub: String,
	pub topic: String,
	pub label: Option<String>,
	pub detail: D,
}

// region:    --- Froms

/// From ModelEvent
///   hub: modelHub
/// topic:  "conv"                       (rel name)
/// label:  "create"                     (crud type name)
/// detail: {rel: "conv", id: 123 }
impl From<ModelEvent> for HubEvent<EntityRef> {
	fn from(model_evt: ModelEvent) -> Self {
		Self {
			hub: s!("modelHub"),
			topic: s!(model_evt.entity_ref().rel),
			label: Some(s!(model_evt.action_name())),
			detail: model_evt.entity_ref().clone(),
		}
	}
}

/// From ConvEvt
impl From<ConvEvent> for HubEvent<Value> {
	fn from(conv_evt: ConvEvent) -> Self {
		let topic = conv_evt.name().to_string();
		// NOTE: For now make the value Null if cannot to_value
		let mut conv_evt_value = serde_json::to_value(conv_evt).unwrap_or(Value::Null);
		let detail = conv_evt_value.x_take("data").unwrap_or(Value::Null);

		Self {
			hub: s!("convHub"),
			topic,
			label: None,
			detail,
		}
	}
}

// endregion: --- Froms
