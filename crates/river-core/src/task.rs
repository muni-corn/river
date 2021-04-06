use chrono::prelude::*;
use diesel::Queryable;
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
    pub id: Option<TaskId>, // no id means it's not in the database
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

type TaskSqlRow = (TaskId, String, DateTime<Utc>, bool, Option<f32>, Option<DateTime<Utc>>);

impl Queryable<crate::schema::tasks::SqlType, crate::Db> for Task {
    type Row = TaskSqlRow;

    fn build(row: Self::Row) -> Self {
        let (id, title, date_added, started, percent_complete_opt, date_completed_opt) = row;
        let status = if let Some(date) = date_completed_opt {
            TaskStatus::Done(date.with_timezone(&Local))
        } else if started {
            if let Some(p) = percent_complete_opt {
                TaskStatus::InProgress(p)
            } else {
                TaskStatus::InProgress(0.0)
            }
        } else {
            TaskStatus::NotStarted
        };

        Self {
            id: Some(id),
            date_added: date_added.with_timezone(&Local),
            title,
            status
        }
    }
}

/// A version of `Task` compatible with an SQL database.
#[derive(Insertable)]
#[table_name = "tasks"]
pub struct InsertableTask {
    title: String,
    date_added: DateTime<Utc>,
    started: bool,
    percent_complete: Option<f32>,
    date_completed: Option<DateTime<Utc>>,
}

impl From<Task> for InsertableTask {
    fn from(t: Task) -> Self {
        Self {
            title: t.title.clone(),
            date_added: t.date_added.with_timezone(&Utc),
            started: t.is_started(),
            percent_complete: t.percent_complete(),
            date_completed: t.date_completed().map(|d| d.with_timezone(&Utc)),
        }
    }
}
