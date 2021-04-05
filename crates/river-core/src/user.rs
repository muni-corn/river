use super::Task;
use serde::{Deserialize, Serialize};

pub type UserId = u64;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    id: UserId,
    status: UserStatus,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

