use crate::{schema::users::SqlType, Db};
use diesel::Queryable;
use river_core::TaskId;
use river_core::UserId;
use river_core::UserStatus;

struct QueryableUser(river_core::User);

type UserSqlRow = (UserId, bool, Option<TaskId>, Option<String>);

impl Queryable<SqlType, Db> for QueryableUser {
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

        Self(river_core::User { id, status })
    }
}
