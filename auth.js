var express = require("express");
var bcrypt = require("bcrypt");
var jwt = require("jsonwebtoken");
var router = express.Router();

const {
    makeGetHashQuery,
    makeCreateUserQuery,
    makeInsertAuthQuery,
    makeGetUserQuery
} = require("./queries");

const newDBClient = require('./db.js');

const saltRounds = 10;

const secret = "weaksecret";

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
    let rows;
    try {
        await client.connect();
        rows = (await client.query(makeGetHashQuery(req.body.email))).rows;
    } catch(e) {
        console.log(e);
        res.status(500).send(e);
    }

    if (rows.length <= 0) {
        res.status(401).send("User not found");
        return;
    }

    const hash = rows[0].hash;

    // authenticate and return jwt if successful
    let token, payload;
    if (await bcrypt.compare(req.body.password, hash)) {
        // get user information
        const userID = rows[0].user;
        let userRows;
        try {
            userRows = (await client.query(makeGetUserQuery(userID))).rows;
        } catch(e) {
            console.log(e);
        }

        if (userRows.length <= 0) {
            res.status(500).send("Couldn't get user by id... for some reason");
            return;
        }

        const user = userRows[0];

        payload = {
            userID: userID,
            firstName: user.first_name,
            lastName: user.last_name,
            displayName: user.display_name
        };

        token = jwt.sign(payload, secret);
    } else {
        res.status(401).send("Incorrect password");
    }

    res.cookie("token", token, { httpOnly: true });
    res.status(200).send(payload);
    await client.end();
});

router.post("/verify", async function(req, res) {
    try {
        await jwt.verify(req.cookies.token, secret);
        res.send(true);
    } catch (e) {
        res.status(401).send(false);
    }
});

router.post("/logout", async function(req, res, next) {
    res.clearCookie("token");
});

module.exports = router;
