var express = require("express");
var jwt = require("jsonwebtoken");
var router = express.Router();

const {
    makeStartQuery,
    makeNewTaskQuery,
    makeStopQuery,
    makeLinkTaskOwnerQuery,
    makePushHistoryQuery,
    makeUpdateTaskQuery,
    makeUpdateHistoryItemQuery,
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
        makeGetHistoryQuery(req.body.userID, req.body.offset, req.body.limit)
    );
    await client.end();
    res.send(result.rows);
});

router.post("/history", async function(req, res) {
    const client = newDBClient();
    const newHistory = await client.query(
        makePushHistoryQuery(
            req.body.userID,
            req.body.title,
            req.body.private,
            req.body.relatedTaskID || null
        )
    ).rows[0];
    await client.end();
    res.send(newHistory);
});

router.get("/tasks", async function(req, res) {
    const client = newDBClient();
    const result = await client.query(makeGetTasksQuery(req.body.userID));
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

module.exports = router;
