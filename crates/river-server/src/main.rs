#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::{net::TcpListener, thread::{spawn, Builder}};
use rocket::{Config, config::Environment, routes};

mod api;
mod db;

use api::*;

fn main() {
    let api_thread_handle = Builder::new().name(String::from("api")).spawn(|| {
        let rocket_config = Config::build(Environment::Staging)
            .port(8800)
            .finalize()
            .unwrap();

        rocket::custom(rocket_config)
            .mount("/", routes![test])
            .launch();
    }).unwrap();

    let ws_thread_handle = Builder::new().name(String::from("websockets")).spawn(|| {
        let ws_server = TcpListener::bind("127.0.0.1:8801").unwrap();
        handle_websockets(ws_server);
    }).unwrap();

    let _ = api_thread_handle.join();
    let _ = ws_thread_handle.join();
}

fn handle_websockets(server: TcpListener) {
    use tungstenite::accept;

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
