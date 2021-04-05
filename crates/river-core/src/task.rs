use chrono::prelude::*;
use crate::schema::tasks;
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

impl Task {
    pub fn is_started(&self) -> bool {
        match self.status {
            TaskStatus::NotStarted => false,
            _ => true
        }
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

/// A version of `Task` compatible with an SQL database.
#[derive(Queryable, Insertable)]
#[table_name = "tasks"]
pub struct SqlTask {
    id: TaskId,
    title: String,
    date_added: DateTime<Utc>,
    started: bool,
    percent_complete: Option<f32>,
    date_completed: Option<DateTime<Utc>>,
}

impl From<Task> for SqlTask {
    fn from(t: Task) -> Self {
        Self {
            id: t.id,
            title: t.title.clone(),
            date_added: t.date_added.with_timezone(&Utc),
            started: t.is_started(),
            percent_complete: t.percent_complete(),
            date_completed: t.date_completed().map(|d| d.with_timezone(&Utc)),
        }
    }
}

impl From<SqlTask> for Task {
    fn from(sql: SqlTask) -> Self {
        let id = sql.id;
        let date_added = sql.date_added.with_timezone(&Local);
        let title = sql.title;
        let status = if let Some(date) = sql.date_completed {
            TaskStatus::Done(date.with_timezone(&Local))
        } else if sql.started {
            if let Some(p) = sql.percent_complete {
                TaskStatus::InProgress(p)
            } else {
                TaskStatus::InProgress(0.0)
            }
        } else {
            TaskStatus::NotStarted
        };

        Self {
            id,
            date_added,
            title,
            status
        }
    }
}
