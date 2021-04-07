use crate::schema::tasks;
use chrono::prelude::*;
use diesel::Queryable;
use river_core::Task;
use river_core::TaskId;
use river_core::TaskStatus;
use river_core::UserId;

struct QueryableTask(Task);

type TaskSqlRow = (
    TaskId,
    String,
    DateTime<Utc>,
    bool,
    Option<f32>,
    Option<DateTime<Utc>>,
    UserId,
);

impl Queryable<crate::schema::tasks::SqlType, crate::Db> for QueryableTask {
    type Row = TaskSqlRow;

    fn build(row: Self::Row) -> Self {
        let (id, title, date_added, started, percent_complete_opt, date_completed_opt, creator_id) =
            row;
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

        Self(Task {
            id: Some(id),
            creator_id,
            date_added: date_added.with_timezone(&Local),
            title,
            status,
        })
    }
}

/// A version of `Task` for inserting into the SQL database
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
