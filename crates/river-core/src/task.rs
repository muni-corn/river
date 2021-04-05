use chrono::prelude::*;
use diesel::prelude::*;
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
    pub id: TaskId,
    pub date_added: DateTime<Local>,
    pub title: String,
    pub status: TaskStatus,
}

type TaskSqlRow = (TaskId, String, chrono::DateTime<Utc>, bool, Option<f32>, Option<chrono::DateTime<Utc>>);

#[derive(Queryable, Insertable)]
#[table_name = "tasks"]
pub struct SqlTask {
    id: TaskId,
    title: String,
    date_added: DateTime<Utc>,
    started: bool,
    percent_complete: f32,
    date_completed: DateTime<Utc>,
}
