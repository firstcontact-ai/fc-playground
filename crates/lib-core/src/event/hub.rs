use crate::event::{ConvEvent, DSourceEvent, ModelEvent};
use crate::event::{Error, Result};
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tracing::error;

/// A construct to manage all the message queues of this lib-core subsystem.
/// - All queues are initialized during Hub initialization.
/// - It follows a Multi Producer Multi Consumer (MPMC) pattern,
///   allowing for multiple subscribers.
/// - Queues are currently implemented using the Flume MPMC crate.
/// - The Hub is designed to be cloneable, as all of its states are in an Arc<...Inner>.
/// - This construct is well-suited for use as an App State.
#[derive(Clone, Default)]
pub struct Hub {
	inner: Arc<HubInner>,
}

/// Inner content of a Hub.
/// Queues are distinguighed by their message type.
/// Note: It is designed to implement a "static dispatch getter" scheme through the GetQueue trait,
///       enhancing API ergonomics without affecting performance
///       (with a negligible increase in code size).
#[derive(Default)]
struct HubInner {
	dsource_queue: Queue<DSourceEvent>,
	conv_queue: Queue<ConvEvent>,
	model_queue: Queue<ModelEvent>,
}

// public Hub functions.
impl Hub {
	/// Provide a convenient API to publish and event directly.
	/// Alternatively, use the `hub.publisher().pub(evt)`
	#[allow(private_bounds)] // ok, GetQueue is just API ergonomics mechanics
	pub async fn publish<M>(&self, msg: M)
	where
		Hub: GetQueue<M>,
	{
		publish_with_tx(&self.get_tx(), msg).await;
	}

	#[allow(private_bounds)] // ok, GetQueue is just internal mechanics
	pub fn subscriber<E>(&self) -> Result<Subscriber<E>>
	where
		E: Clone,
		Hub: GetQueue<E>,
	{
		let rx = self.get_rx();
		Ok(Subscriber::new(rx))
	}

	#[allow(private_bounds)] // ok, GetQueue is just internal mechanics
	pub fn publisher<M>(&self) -> Result<Publisher<M>>
	where
		Hub: GetQueue<M>,
	{
		let tx = self.get_tx();
		Ok(Publisher::new(tx))
	}
}

// private hub functions
impl Hub {
	fn get_rx<M>(&self) -> Receiver<M>
	where
		Hub: GetQueue<M>,
	{
		self.get_queue().tx.subscribe()
	}

	fn get_tx<M>(&self) -> Sender<M>
	where
		Hub: GetQueue<M>,
	{
		self.get_queue().tx.clone()
	}
}

// region:    --- Subscriber

pub struct Subscriber<E: Clone> {
	rx: Receiver<E>,
}

impl<E: Clone> Subscriber<E> {
	fn new(rx: Receiver<E>) -> Self {
		Subscriber { rx }
	}

	pub async fn next(&mut self) -> Result<E> {
		let m = self.rx.recv().await.map_err(|re| Error::Receive(re.to_string()))?;
		Ok(m)
	}
}

// endregion: --- Subscriber

// region:    --- Publisher

#[derive(Clone)]
pub struct Publisher<E> {
	tx: Sender<E>,
}

impl<E> Publisher<E> {
	fn new(tx: Sender<E>) -> Self {
		Publisher { tx }
	}

	pub async fn publish(&self, evt: E) {
		publish_with_tx(&self.tx, evt).await
	}
}

/// publish support function to be use in Publisher and Hub
async fn publish_with_tx<E>(tx: &Sender<E>, evt: E) {
	if let Err(err) = tx.send(evt).map_err(|se| Error::Send(se.to_string())) {
		error!("FAIL - Hub publish failed. Cause: {err}");
	}
}

// endregion: --- Publisher

// region:    --- Queue

struct Queue<E> {
	tx: Sender<E>,
	rx: Receiver<E>, // to make sure it does not close
}

/// impl default
// impl Default
impl<E: Clone> Default for Queue<E> {
	fn default() -> Self {
		let (tx, rx) = broadcast::channel::<E>(16);
		Queue { tx, rx }
	}
}

trait GetQueue<M> {
	fn get_queue(&self) -> &Queue<M>;
}

// endregion: --- Queue

// region:    --- GetQueue Implementations

impl GetQueue<DSourceEvent> for Hub {
	fn get_queue(&self) -> &Queue<DSourceEvent> {
		&self.inner.dsource_queue
	}
}

impl GetQueue<ConvEvent> for Hub {
	fn get_queue(&self) -> &Queue<ConvEvent> {
		&self.inner.conv_queue
	}
}

impl GetQueue<ModelEvent> for Hub {
	fn get_queue(&self) -> &Queue<ModelEvent> {
		&self.inner.model_queue
	}
}

// endregion: --- GetQueue Implementations
