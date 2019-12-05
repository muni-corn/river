const { Client } = require("pg");

function newDBClient() {
    return new Client({
        connectionString: process.env.DATABASE_URL
    });
}

module.exports = newDBClient;
