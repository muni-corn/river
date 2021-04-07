use chrono::prelude::*;
use crate::UserId;
use serde::{Deserialize, Serialize};

pub type TaskId = i32;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TaskStatus {
    /// The task hasn't been started on yet, much like this project 75% through my last semester in
    /// college.
    NotStarted,

    /// The task is in progress, with a specified percentage (from 0.0 to 1.0) complete.
    InProgress(f32),

    /// The task is done. The DateTime specifies when it was marked as done.
    Done(chrono::DateTime<Local>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub id: Option<TaskId>, // no id means it's not in the database yet
    pub creator_id: UserId,
    pub date_added: DateTime<Local>,
    pub title: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn is_started(&self) -> bool {
        !matches!(self.status, TaskStatus::NotStarted)
    }

    pub fn percent_complete(&self) -> Option<f32> {
        match self.status {
            TaskStatus::Done(_) => Some(1.0),
            TaskStatus::InProgress(p) => Some(p),
            TaskStatus::NotStarted => None,
        }
    }

    pub fn date_completed(&self) -> Option<chrono::DateTime<Local>> {
        match self.status {
            TaskStatus::Done(date) => Some(date),
            _ => None,
        }
    }
}
