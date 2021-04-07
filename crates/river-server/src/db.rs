use crate::schema;
use diesel::{
    pg::PgConnection,
    prelude::{ConnectionError, ExpressionMethods, QueryDsl, RunQueryDsl},
    Connection,
};
use dotenv::dotenv;
use river_core::{TaskId, UserId};
use std::env;

pub fn connect() -> DbResult<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL needs to be set in .env");

    PgConnection::establish(&database_url).map_err(DbError::Connection)
}

fn set_user_working(new_task_id: Option<TaskId>, user_id: UserId) -> DbResult<()> {
    use schema::users::dsl::*;

    let connection = crate::db::connect()?;

    diesel::update(users.find(user_id))
        .set((
            present.eq(true),
            current_task.eq(new_task_id),
            current_away_reason.eq(None::<String>),
        ))
        .execute(&connection)
        .map_err(DbError::Transaction)?;

    Ok(())
}

fn set_user_away(reason: String, user_id: UserId) -> DbResult<()> {
    use schema::users::dsl::*;

    let connection = crate::db::connect()?;

    diesel::update(users.find(user_id))
        .set((current_away_reason.eq(reason), present.eq(true)))
        .execute(&connection)
        .map_err(DbError::Transaction)?;

    Ok(())
}

fn set_user_out(user_id: UserId) -> DbResult<()> {
    use schema::users::dsl::*;

    let connection = crate::db::connect()?;

    diesel::update(users.find(user_id))
        .set(present.eq(false))
        .execute(&connection)
        .map_err(DbError::Transaction)?;

    Ok(())
}

// fn makePushHistoryQuery(userID, actionName, private, relatedTaskID) {
//     // return {
//     //     name: "pushHistory",
//     //     text:
//     //         'INSERT INTO "public.history" ' +
//     //         '("user", "action", "time", "private", "related_task") ' +
//     //         "VALUES " +
//     //         "($1, $2, NOW(), $3, $4) RETURNING *;",
//     //     values: [userID, actionName, private, relatedTaskID]
//     // };
// }

// fn makeNewTaskQuery(userID, taskTitle, private) {
//     // return {
//     //     name: "newTask",
//     //     text:
//     //         'INSERT INTO "public.task" (owner, name, creation_date, private) VALUES ' +
//     //         "($1, $2, NOW(), $3) RETURNING *;",
//     //     values: [userID, taskTitle, private]
//     // };
// }

// fn makeGetHistoryQuery(userID, offset, limit) {
//     // return {
//     //     name: "getHistory",
//     //     text:
//     //         'SELECT * FROM "public.history" WHERE "user" = $1 OFFSET $2 LIMIT $3;',
//     //     values: [userID, offset, limit]
//     // };
// }

// fn makeUpdateTaskQuery(
//     // taskID,
//     // name,
//     // percentComplete,
//     // minutesSpent,
//     // wasCompletedAt,
//     // private
// ) {
//     // return {
//     //     name: "updateTask",
//     //     text:
//     //         'UPDATE "public.task" SET ' +
//     //         '"name" = $1,' +
//     //         '"percent_complete" = $2,' +
//     //         '"minutes_spent" = $3,' +
//     //         '"was_completed_at" = $4,' +
//     //         '"private" = $5 ' +
//     //         'WHERE "id" = $6 RETURNING *;',
//     //     values: [
//     //         name,
//     //         percentComplete,
//     //         minutesSpent,
//     //         wasCompletedAt,
//     //         private,
//     //         taskID
//     //     ]
//     // };
// }

// fn make_update_history_item_query(historyID, private) {
//     // return {
//     //     name: "updateHistoryItem",
//     //     text: 'UPDATE "public.history" SET "private" = $1 WHERE "id" = $2;',
//     //     values: [private, historyID]
//     // };
// }

// fn makeGetTasksQuery(userID) {
//     // return {
//     //     name: "getTasks",
//     //     text: 'SELECT * FROM "public.task" WHERE "owner" = $1;',
//     //     values: [userID]
//     // };
// }

// fn makeGetIncompleteTasksQuery(userID) {
//     // return {
//     //     name: "getTodoTasks",
//     //     text:
//     //         'SELECT * FROM "public.task" WHERE "owner" = $1 AND "was_completed_at" IS NULL;',
//     //     values: [userID]
//     // };
// }

// fn makeGetSingleTaskQuery(taskID) {
//     // return {
//     //     name: "getSingleTask",
//     //     text: 'SELECT * FROM "public.task" WHERE "id" = $1;',
//     //     values: [taskID]
//     // };
// }

// fn makeGetHashQuery(email) {
//     // return {
//     //     name: "getHash",
//     //     text:
//     //         'SELECT "user", "hash" FROM "public.auth" WHERE "user" = (SELECT id FROM "public.user" WHERE "email" = $1);',
//     //     values: [email]
//     // };
// }

// fn makeGetUserQuery(userID) {
//     // return {
//     //     name: "getUser",
//     //     text:
//     //         'SELECT "first_name", "last_name", "display_name" FROM "public.user" WHERE "id" = $1;',
//     //     values: [userID]
//     // };
// }

// fn makeCreateUserQuery(firstName, lastName, email, displayName) {
//     // return {
//     //     name: "createUser",
//     //     text:
//     //         'INSERT INTO "public.user" (first_name, last_name, email, display_name) ' +
//     //         "VALUES ($1, $2, $3, $4) RETURNING id;",
//     //     values: [firstName, lastName, email, displayName]
//     // };
// }

// fn makeInsertAuthQuery(userID, hash) {
//     // return {
//     //     name: "insertAuth",
//     //     text: 'INSERT INTO "public.auth" ("user", hash) ' + "VALUES ($1, $2);",
//     //     values: [userID, hash]
//     // };
// }

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug)]
enum DbError {
    Connection(diesel::result::ConnectionError),
    Transaction(diesel::result::Error),
}

impl From<ConnectionError> for DbError {
    fn from(e: ConnectionError) -> Self {
        Self::Connection(e)
    }
}
