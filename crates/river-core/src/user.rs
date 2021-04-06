use crate::{Db, schema::users::SqlType};
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use super::TaskId;

pub type UserId = i32;

/// A user.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    id: UserId,
    status: UserStatus,
}

type UserSqlRow = (UserId, bool, Option<TaskId>, Option<String>);

impl Queryable<SqlType, Db> for User {
    type Row = UserSqlRow;

    fn build(row: Self::Row) -> Self {
        let (id, present, current_task_id, current_away_reason_opt) = row;

        let status = if present { 
            if let Some(current_away_reason) = current_away_reason_opt {
                UserStatus::Away(current_away_reason)
            } else {
                UserStatus::Working(current_task_id)
            }
        } else {
            UserStatus::Out
        };

        Self {
            id,
            status,
        }
    }
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

