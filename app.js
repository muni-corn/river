require("dotenv").config();

var express = require("express");
var path = require("path");
var cookieParser = require("cookie-parser");
var logger = require("morgan");
var jwt = require("jsonwebtoken");

var apiRouter = require("./api.js");
var authRouter = require("./auth.js");

var app = express();

var secret = "weaksecret";

app.use(logger("dev"));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());

async function authMiddleware(req, res, next) {
    try {
        const token = req.cookies.token;
        const { userID } = jwt.verify(token, secret);
        req.userID = userID;
        return next();
    } catch (e) {
        console.dir(e);
        res.status(401).redirect("/auth/login");
        return;
    }
}

app.use("/api", authMiddleware, apiRouter);
app.use("/auth", authRouter);
app.use(express.static(path.join(__dirname, "dist")));
app.use(function(_, res) {
    res.sendFile(path.join(__dirname, "dist/index.html"));
});

module.exports = app;
