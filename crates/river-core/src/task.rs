use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub type TaskId = u64;

#[derive(Deserialize, Serialize, Clone)]
pub enum TaskStatus {
    NotStarted,

    /// The task is in progress, with a specified percentage (from 0.0 to 1.0) complete.
    InProgress(f32),

    /// The task is done. The DateTime specifies when it was marked as done.
    Done(chrono::DateTime<Local>),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: TaskId,
    pub date_added: DateTime<Local>,
    pub title: String,
    pub status: TaskStatus,
}
