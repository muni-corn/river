use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Clone)]
pub enum TaskStatus {
    NotStarted,
    
    /// The task is in progress, with a specified percentage (from 0.0 to 1.0) complete.
    InProgress(f32),

    /// The task is done. The DateTime specifies when it was marked as done.
    Done(chrono::DateTime<Local>),
}

#[derive(Clone)]
pub struct Task {
    pub date_added: DateTime<Local>,
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Clone)]
pub enum UserStatus {
    // The user is working on a Task
    Working(Task),

    // The user is taking a break, with a given reason
    Away(String),

    // The user isn't present
    Out,
}

/// Like `UserStatus`, but without guts
#[derive(Clone, Copy, Debug)]
pub enum UserStatusCategory {
    Working,
    Away,
    Out,
}

impl UserStatusCategory {
    pub fn display(&self) -> String {
        String::from(match self {
            UserStatusCategory::Working => "Working",
            UserStatusCategory::Away => "Taking a break",
            UserStatusCategory::Out => "Out",
        })
    }
}

impl From<UserStatus> for UserStatusCategory {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Working(_) => Self::Working,
            UserStatus::Away(_) => Self::Away,
            UserStatus::Out => Self::Out,
        }
    }
}

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
struct WebSocketRequest {
    value: u32,
}
