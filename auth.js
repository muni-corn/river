var express = require("express");
var bcrypt = require("bcrypt");
var jwt = require("jsonwebtoken");
var router = express.Router();

const {
    makeGetHashQuery,
    makeCreateUserQuery,
    makeInsertAuthQuery
} = require("./queries");

const newDBClient = require('./db.js');

const saltRounds = 10;

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
    const rows = (await client.query(makeGetHashQuery(req.body.email))).rows;

    if (rows.length <= 0) {
        res.status(401).send("User not found");
        return;
    }

    const hash = rows[0].hash;

    // authenticate and return jwt if successful
    let token;
    if (await bcrypt.compare(req.body.password, hash)) {
        token = jwt.sign({ userID: rows[0].user }, process.env.SECRET);
    } else {
        res.status(401).send("Incorrect password");
    }

    res.cookie("token", token, { httpOnly: true });
    res.send(token);
    await client.end();
});

module.exports = router;
