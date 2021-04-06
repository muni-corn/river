use river_core::UserId;

#[get("/test/<user_id>")]
pub fn test(user_id: UserId) -> String {
    println!("userid: {}", user_id);

    String::from("It works!")
}

// #[post("/start")]
// fn start_task() {
//     let client = newDBClient();
//     client.connect();
//     let updated = client.query(
//         makeStartQuery(task_id, user_id)
//     ).rows[0];
//     client.end();
//     res.send(updated);
// }

// /// Make the user's status "out"
// #[post("/stop")]
// fn stop_working() {
//     let client = newDBClient();
//     client.connect();
//     let updated = client.query(
//         makeStopQuery(reason, user_id)
//     ).rows[0];
//     client.end();
//     res.send(updated);
// }

// #[post("/new_task")]
// fn new_task() {
//     let client = newDBClient();
//     client.connect();
//     let rawTask = client.query(makeNewTaskQuery(user_id, name, priv)).rows[0];
//     res.status(200).send(rawTask);
//     client.end();
// }

// #[post("/update_task")]
// fn update_task() {
//     let client = newDBClient();
//     client.connect();
//     let {
//         id,
//         name,
//         percentComplete,
//         minutesSpent,
//         wasCompletedAt,
//         private
//     } = task;
//     let newTask = client.query(
//         makeUpdateTaskQuery(
//             id,
//             name,
//             percentComplete,
//             minutesSpent,
//             wasCompletedAt,
//             private
//         )
//     ).rows[0];
//     client.end();
//     res.send(new_task);
// }

// #[post("/update_history_item")]
// fn update_history(history_id: HistoryId, make_private: bool) {
//     let client = newDBClient();
//     client.connect();
//     let newHistoryItem = client.query(
//         make_update_history_item_query(history_id, make_private)
//     ).rows[0];
//     client.end();
//     res.send(newHistoryItem);
// }

// #[get("/history")]
// fn get_history() {
//     let client = newDBClient();
//     client.connect();
//     let result = client.query(
//         makeGetHistoryQuery(user_id, offset, limit)
//     );
//     client.end();
//     res.send(result.rows);
// }

// #[post("/history")]
// fn new_history() {
//     let client = newDBClient();
//     try {
//         client.connect();
//         let result = client.query(
//             makePushHistoryQuery(
//                 user_id,
//                 title,
//                 private,
//                 relatedTask_id || null
//             )
//         );
//         res.send(result);
//     } catch (e) {
//         res.status(500).send(e);
//     } finally {
//         client.end();
//     }
// }

// #[get("/tasks")]
// fn get_tasks() {
//     let client = newDBClient();
//     client.connect();
//     let result = client.query(makeGetTasksQuery(user_id));
//     client.end();
//     res.send(result.rows);
// }

// #[get("/singleTask")]
// fn get_single_task() {
//     let client = newDBClient();
//     client.connect();
//     let task = client.query(makeGetSingleTaskQuery(task_id))
//         .rows[0];
//     client.end();
//     res.send(task);
// }

// #[get("/user")]
// fn get_user(user_id: UserId) {
//     let client = newDBClient();
//     try {
//         client.connect();

//         let user = (client.query(makeGetUserQuery(user_id))).rows[0];
//         let todo = (
//             client.query(makeGetIncompleteTasksQuery(user_id))
//         ).rows;
//         let history = (
//             client.query(makeGetHistoryQuery(user_id, 0, 10))
//         ).rows;
//         let result = {
//             user,
//             todo,
//             history
//         };
//         res.status(200).send(result);
//         return;
//     } catch (e) {
//         res.status(500).send(e);
//     } finally {
//         client.end();
//     }
// }
