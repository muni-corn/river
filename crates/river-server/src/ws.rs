use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::SinkExt;
use futures::StreamExt;
use river_core::{UserAction, UserId, WebSocketRequest, WebSocketResponseBody};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub async fn start_ws_server() {
    println!("starting websockets server");
    let ws_server = TcpListener::bind("127.0.0.1:8801").await.unwrap();
    println!("websockets server is running");
    handle_websockets(ws_server).await;
}

type UserEventSubscribers = Arc<Mutex<Vec<UnboundedSender<(UserId, UserAction)>>>>;
type OutgoingSinkMutex = Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>;

async fn handle_stream(stream: TcpStream, user_subs_mutex: UserEventSubscribers) {
    use tokio_tungstenite::accept_async;

    println!("handling stream");

    let websocket = accept_async(stream).await.unwrap();
    let (outgoing, incoming) = websocket.split();
    let outgoing_mutex = Arc::new(Mutex::new(outgoing));

    let user_ids = HashSet::<UserId>::new();
    let user_ids_mutex = Arc::new(Mutex::new(user_ids));

    let (tx, rx) = mpsc::unbounded_channel::<(UserId, UserAction)>();
    user_subs_mutex.lock().unwrap().push(tx);

    let outgoing_mutex_clone = outgoing_mutex.clone();
    let user_subs_mutex_clone = user_subs_mutex.clone();
    let user_ids_mutex_clone = user_ids_mutex.clone();
    let incoming_handle = std::thread::spawn(move || {
        handle_incoming(
            incoming,
            outgoing_mutex_clone,
            user_subs_mutex_clone,
            user_ids_mutex_clone,
        )
    });
    let outgoing_handle =
        std::thread::spawn(move || handle_outgoing(outgoing_mutex, rx, user_ids_mutex));

    incoming_handle.join();
    outgoing_handle.join();
}

async fn handle_websockets(server: TcpListener) {
    let user_subscription_senders = Vec::<UnboundedSender<(UserId, UserAction)>>::new();
    let user_subscription_senders_mutex = Arc::new(Mutex::new(user_subscription_senders));

    while let Ok((stream, _)) = server.accept().await {
        let user_subs_mutex = user_subscription_senders_mutex.clone();

        tokio::spawn(handle_stream(stream, user_subs_mutex));
    }
}

fn update_user_subscribers(
    user_subs_mutex: UserEventSubscribers,
    user_id: UserId,
    action: UserAction,
) {
    println!("updating user subscribers");
    let subs = user_subs_mutex.lock().unwrap();

    for sub in subs.iter() {
        let _ = sub.send((user_id, action.clone()));
    }
}

fn handle_incoming(
    incoming: SplitStream<WebSocketStream<TcpStream>>,
    outgoing: OutgoingSinkMutex,
    user_subs_mutex: UserEventSubscribers,
    user_ids: Arc<Mutex<HashSet<UserId>>>,
) {
    println!("starting incoming handler");
    futures::executor::block_on(
        incoming.for_each(|inp| async {
            println!("handling incoming: {:?}", inp);
            handle_message(
                inp,
                outgoing.clone(),
                user_subs_mutex.clone(),
                user_ids.clone(),
            )
            .await
        })
    );
}

async fn handle_message(
    inp: Result<Message, Error>,
    outgoing: OutgoingSinkMutex,
    user_subs_mutex: UserEventSubscribers,
    user_ids: Arc<Mutex<HashSet<UserId>>>,
) {
    println!("handling message: {:?}", inp);
    if let Ok(msg) = inp {
        println!("{:#?}", msg);
        if let Message::Text(s) = msg {
            let req: WebSocketRequest = serde_json::from_str(&s).unwrap();
            match req {
                WebSocketRequest::SubscribeToUser(user_id) => {
                    println!("registering an id for subscription");
                    user_ids.lock().unwrap().insert(user_id);
                    outgoing
                        .lock()
                        .unwrap()
                        .send(Message::Text(
                            serde_json::to_string(&WebSocketResponseBody::SubscribeSuccessful)
                                .unwrap(),
                        ))
                        .await;
                }
                WebSocketRequest::UpdateUser(user_id, action) => {
                    println!("updating subscribers");
                    update_user_subscribers(user_subs_mutex.clone(), user_id, action)
                }
            }
        }
    }
}

fn handle_outgoing(
    outgoing: OutgoingSinkMutex,
    mut rx: UnboundedReceiver<(UserId, UserAction)>,
    user_ids: Arc<Mutex<HashSet<UserId>>>,
) {
    println!("starting outgoing handler");

    while let Some((id, action)) = rx.blocking_recv() {
        println!("subscriber received update");
        if user_ids.lock().unwrap().iter().any(|i| *i == id) {
            println!("sending update to websocket");
            futures::executor::block_on(
                outgoing
                    .lock()
                    .unwrap()
                    .send(Message::Text(
                        serde_json::to_string(&WebSocketResponseBody::UserUpdate(
                            id,
                            action.clone(),
                        ))
                        .unwrap(),
                    ))
            );
        }
    }
    println!("subscriber quit listening");
}
