use serde::{Deserialize, Serialize};

use crate::TaskId;

pub type UserId = i32;

/// A user.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub status: UserStatus,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum UserStatus {
    /// The user is working, most likely on a Task. If the [`TaskId`](TaskId) is None, the user is probably
    /// awaiting work to do.
    Working(Option<TaskId>),

    /// The user is taking a break, with a given reason
    Away(String),

    /// The user isn't present
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

