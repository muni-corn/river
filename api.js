var express = require("express");
var router = express.Router();

const {
    makeStartQuery,
    makeNewTaskQuery,
    makeStopQuery,
    makeLinkTaskOwnerQuery,
    makePushHistoryQuery,
    makeUpdateTaskQuery,
    makeUpdateHistoryItemQuery,
    makeGetUserQuery,
    makeGetIncompleteTasksQuery
} = require("./queries");

const newDBClient = require('./db.js');

router.get("/test", function(req, res) {
    console.log("userid:", req.userID);
    res.send("It works!");
});

router.post("/start", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    const updated = await client.query(
        makeStartQuery(req.body.taskID, req.body.userID)
    ).rows[0];
    await client.end();
    res.send(updated);
});

router.post("/stop", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    const updated = await client.query(
        makeStopQuery(req.body.reason, req.body.userID)
    ).rows[0];
    await client.end();
    res.send(updated);
});

router.post("/newTask", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    try {
        await client.query("BEGIN");
        const task = (
            await client.query(
                makeNewTaskQuery(req.body.taskTitle, req.body.private)
            )
        ).rows[0];
        await client.query(makeLinkTaskOwnerQuery(req.body.userID, task.id));
        await client.query("COMMIT");
        res.send(task);
    } catch (e) {
        await client.query("ROLLBACK");
        res.status(500).send(e);
    } finally {
        await client.end();
    }
});

router.post("/updateTask", async function(req, res) {
    const client = newDBClient();
    const {
        id,
        name,
        percentComplete,
        minutesSpent,
        wasCompletedAt,
        private
    } = req.body.task;
    const newTask = await client.query(
        makeUpdateTaskQuery(
            id,
            name,
            percentComplete,
            minutesSpent,
            wasCompletedAt,
            private
        )
    ).rows[0];
    await client.end();
    res.send(newTask);
});

router.post("/updateHistoryItem", async function(req, res) {
    const client = newDBClient();
    const newHistoryItem = await client.query(
        makeUpdateHistoryItemQuery(req.body.historyID, req.body.private)
    ).rows[0];
    await client.end();
    res.send(newHistoryItem);
});

router.get("/history", async function(req, res) {
    const client = newDBClient();
    const result = await client.query(
        makeGetHistoryQuery(req.userID, req.body.offset, req.body.limit)
    );
    await client.end();
    res.send(result.rows);
});

router.post("/history", async function(req, res) {
    const client = newDBClient();
    let newHistory;
    try {
        client.query(
            makePushHistoryQuery(
                req.userID,
                req.body.title,
                req.body.private,
                req.body.relatedTaskID || null
            )
        ).then(result => { // await doesn't work for some reason
            client.end();
            res.send(result);
        }, err => {
            res.status(500).send(err);
        });
    } catch (e) {
        res.status(500).send(e);
    }
    res.send(newHistory);
});

router.get("/tasks", async function(req, res) {
    const client = newDBClient();
    const result = await client.query(makeGetTasksQuery(req.userID));
    await client.end();
    res.send(result.rows);
});

router.get("/singleTask", async function(req, res) {
    const client = newDBClient();
    const task = await client.query(makeGetSingleTaskQuery(req.body.taskID))
        .rows[0];
    await client.end();
    res.send(task);
});

router.get("/user", async function(req, res) {
    const client = newDBClient();
    try {
        const user = (await client.query(makeGetUserQuery(req.userID))).rows[0];
        const todo = (await client.query(makeGetIncompleteTasksQuery(req.userID))).rows;
        const history = (await client.query(makeGetHistoryQuery(req.userID, 0, 10))).rows;
        const userMapped = {
            displayName: user.display_name,
            firstName: user.first_name,
            lastName: user.last_name
        };
        const tasksMapped = todo.map(task => {
            return {
                id: task.id,
                name: task.name,
                percentComplete: task.percent_complete,
                minutesSpent: task.minutes_spent,
                wasCompletedAt: task.was_completed_at,
                creationDate: task.creation_date
            };
        });
        const historyMapped = history.map(item => {
            return {
                at: item.time,
                title: item.action,
                priv: item.private,
                relatedTaskID: item.related_task
            };
        })
        const result = {
            userName: userMapped.displayName || userMapped.firstName + (userMapped.lastName ? ` ${userMapped.lastName}` : ""),
            currentTaskID: user.current_task,
            awayReason: user.away_reason,
            todo: tasksMapped,
            history: historyMapped
        };
        res.send(result);
        return;
    } catch (e) {
        res.status(500).send(e);
    }
});

module.exports = router;
