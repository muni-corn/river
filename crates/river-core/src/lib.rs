use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub mod task;
pub mod user;

pub use task::*;
pub use user::*;

pub type AsBinary = bool;

#[derive(Debug)]
pub enum WebSocketAction {
    Connect,
    Disconnect,
    Send(WebSocketRequest),
    Terminated,
}

/// This type is an expected response from a websocket connection.
#[derive(Serialize, Deserialize, Debug)]
pub enum WebSocketResponseBody {
    SubscribeSuccessful,
    UserUpdate(UserId, UserAction)
}

pub type WebSocketResponse = Result<WebSocketResponseBody, anyhow::Error>;

/// This type is used as a request which sent to websocket connection.
#[derive(Deserialize, Serialize, Debug)]
pub enum WebSocketRequest {
    SubscribeToUser(UserId),
    UpdateUser(UserId, UserAction),

}

/// An error with a message.
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum UserAction {
    ChangeTask(Option<Task>),
    SetAway(String),
    SetOut,
    SetPrivate,
}
