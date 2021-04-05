use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod task;
mod user;

pub use task::*;


pub type AsBinary = bool;

#[derive(Debug)]
pub enum WebSocketAction {
    Connect,
    Disconnect,
    Send(AsBinary),
    Terminated,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WebSocketResponseBody {
    value: u32
}

pub type WebSocketResponse = Result<WebSocketResponseBody, anyhow::Error>;

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
pub struct WebSocketRequest {
    value: u32,
}

/// An error with a message.
#[derive(Deserialize, Debug)]
pub struct WebSocketError(String);

impl From<anyhow::Error> for WebSocketError {
    fn from(e: anyhow::Error) -> Self {
        Self(e.to_string())
    }
}

impl Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error with websockets: {}", self.0)
    }
}
