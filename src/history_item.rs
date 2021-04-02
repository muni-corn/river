use crate::{TaskId, UserId};

pub enum HistoryItem {
    /// The user added a task
    TaskAdded(TaskId),

    /// The user renamed a task
    TaskRenamed(TaskId),

    /// The user deleted a task. Contains the name of the task.
    TaskDeleted(String),

    /// The user switched to or started the task
    WorkStarted(TaskId),

    /// The user started a break
    BreakStarted(String),

    /// The user left
    ClockedOut,

    /// Something else.
    Other(String),
}
