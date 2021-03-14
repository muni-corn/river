use chrono::prelude::*;

pub enum TaskStatus {
    NotStarted,
    
    /// The task is in progress, with a specified percentage (from 0.0 to 1.0) complete.
    InProgress(f32),

    /// The task is done. The DateTime specifies when it was marked as done.
    Done(chrono::DateTime<Local>),
}

pub struct Task {
    date_added: DateTime<Local>,
    title: String,
    status: TaskStatus,
}
