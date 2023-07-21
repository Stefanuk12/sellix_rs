// Dependencies
use serde::{Deserialize, Serialize};
use crate::events::Event;

/// The base request from a Sellix webhook.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawWebsocketRequest<T> {
    event: Event,
    data: T
}