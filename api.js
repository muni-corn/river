var express = require("express");
var bcrypt = require("bcrypt");
var router = express.Router();

const { Client } = require("pg");
const {
    makeStartQuery,
    makeNewTaskQuery,
    makeStopQuery,
    makeLinkTaskOwnerQuery,
    makePushHistoryQuery,
    makeUpdateTaskQuery,
    makeUpdateHistoryItemQuery,
    makeGetHashQuery,
    makeCreateUserQuery,
    makeInsertAuthQuery
} = require("./queries");

function newDBClient() {
    return new Client({
        connectionString: process.env.DATABASE_URL
    });
}

router.get("/test", function(_, res) {
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

// auth and users

const saltRounds = 10

router.post("/register", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    try {
        await client.query("BEGIN");

        // make user
        const userID = (
            await client.query(
                makeCreateUserQuery(
                    req.body.firstName,
                    req.body.lastName,
                    req.body.email,
                    req.body.displayName
                )
            )
        ).rows[0].id; // stupid, but if no rows are returned that's also kinda dumb

        // make password hash
        const hash = await bcrypt.hash(req.body.password, saltRounds);

        // insert hash into auth table
        await client.query(makeInsertAuthQuery(userID, hash));
        await client.query("COMMIT");
        res.send();
    } catch (e) {
        console.log(e);
        await client.query("ROLLBACK");
        res.status(500).send(e);
    } finally {
        await client.end();
    }
});

router.post("/login", async function(req, res) {
    const client = newDBClient();
    await client.connect();
    const rows = (
        await client.query(
            makeGetHashQuery(req.body.email)
        )
    ).rows;

    if (rows.length <= 0) {
        res.status(401).send("User not found");
        return;
    }

    const hash = rows[0].hash;

    // authenticate and return jwt if successful
    let token;
    if (await bcrypt.compare(req.body.password, hash)) {

    } else {
        res.status(401).send("Incorrect password");
    }

    res.send(token);
    await client.end();
});

module.exports = router;
