require("dotenv").config();

var express = require("express");
var path = require("path");
var cookieParser = require("cookie-parser");
var logger = require("morgan");

var apiRouter = require("./api.js");

var app = express();

app.use(logger("dev"));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());

app.use("/api", apiRouter);
app.use(express.static(path.join(__dirname, "dist")));
app.use(function(_, res) {
    res.sendFile(path.join(__dirname, "dist/index.html"));
});

module.exports = app;
