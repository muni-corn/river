use std::{net::TcpListener, thread::spawn};

use tungstenite::accept;

mod api;
mod db;
mod queries;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    rocket::ignite().mount("/", routes![hello]).launch();
    handle_websockets(server);
}

fn handle_websockets(server: TcpListener) {
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
