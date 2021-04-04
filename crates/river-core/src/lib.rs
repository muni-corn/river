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
