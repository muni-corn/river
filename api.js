var express = require("express");
var router = express.Router();

const {
    makeStartQuery,
    makeNewTaskQuery,
    makeStopQuery,
    makePushHistoryQuery,
    makeUpdateTaskQuery,
    makeUpdateHistoryItemQuery,
    makeGetUserQuery,
    makeGetIncompleteTasksQuery,
    makeGetHistoryQuery
} = require("./queries");

const newDBClient = require("./db.js");

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
    try {
        await client.connect();
        const rawTask = (
            await client.query(
                makeNewTaskQuery(req.userID, req.body.name, req.body.priv)
            )
        ).rows[0];
        res.status(200).send(rawTask);
    } catch (e) {
        res.status(500).send(e);
    } finally {
        await client.end();
    }
});

router.post("/updateTask", async function(req, res) {
    const client = newDBClient();
    await client.connect();
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
    await client.connect();
    const newHistoryItem = await client.query(
        makeUpdateHistoryItemQuery(req.body.historyID, req.body.private)
    ).rows[0];
    await client.end();
    res.send(newHistoryItem);
});

router.get("/history", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    const result = await client.query(
        makeGetHistoryQuery(req.userID, req.body.offset, req.body.limit)
    );
    await client.end();
    res.send(result.rows);
});

router.post("/history", async function(req, res) {
    const client = newDBClient();
    try {
        await client.connect();
        const result = await client.query(
            makePushHistoryQuery(
                req.userID,
                req.body.title,
                req.body.private,
                req.body.relatedTaskID || null
            )
        );
        res.send(result);
    } catch (e) {
        res.status(500).send(e);
    } finally {
        await client.end();
    }
});

router.get("/tasks", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    const result = await client.query(makeGetTasksQuery(req.userID));
    await client.end();
    res.send(result.rows);
});

router.get("/singleTask", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    const task = await client.query(makeGetSingleTaskQuery(req.body.taskID))
        .rows[0];
    await client.end();
    res.send(task);
});

router.get("/user", async function(req, res) {
    const client = newDBClient();
    try {
        await client.connect();

        const user = (await client.query(makeGetUserQuery(req.userID))).rows[0];
        const todo = (
            await client.query(makeGetIncompleteTasksQuery(req.userID))
        ).rows;
        const history = (
            await client.query(makeGetHistoryQuery(req.userID, 0, 10))
        ).rows;
        const result = {
            user,
            todo,
            history
        };
        res.status(200).send(result);
        return;
    } catch (e) {
        res.status(500).send(e);
    } finally {
        await client.end();
    }
});

module.exports = router;
