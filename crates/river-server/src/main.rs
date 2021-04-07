#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod api;
mod db;
mod models;
mod schema;
mod ws;

use api::*;
use rocket::{config::Environment, routes, Config};

type Db = diesel::pg::Pg;

#[tokio::main]
async fn main() {
    let api_thread_handle = tokio::spawn(start_api_server());
    let ws_thread_handle = tokio::spawn(ws::start_ws_server());

    tokio::join!(api_thread_handle, ws_thread_handle);
}

async fn start_api_server() {
    let rocket_config = Config::build(Environment::Staging)
        .port(8800)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .mount("/api/", routes![test])
        .launch();
}

