use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

use crate::config::{Config, Server};
use crate::db::EventCollection;

type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

pub struct SocketServer {
    event_collection: &'static Arc<Mutex<EventCollection>>,
    listener: TcpListener,
    state: PeerMap,
}

impl SocketServer {
    pub async fn create(
        config: &Config,
        collection: &'static Arc<Mutex<EventCollection>>,
    ) -> SocketServer {
        let Server { addr, port } = &config.server;
        let server_addr = format!("{}:{}", addr, port);
        let listener = TcpListener::bind(server_addr).await.unwrap();
        SocketServer {
            event_collection: collection,
            listener,
            state: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl SocketServer {
    pub async fn listen(&self) {
        println!("listen to new connections..");
        while let Ok((stream, addr)) = self.listener.accept().await {
            tokio::spawn(handle_connection(
                self.state.clone(),
                self.event_collection,
                stream,
                addr,
            ));
        }
    }
}

async fn handle_connection(
    state: PeerMap,
    event_collection: &'static Arc<Mutex<EventCollection>>,
    stream: TcpStream,
    addr: SocketAddr,
) {
    let mut service_name: Option<HeaderValue> = None;

    let header_callback =
        |request: &Request, response: Response| -> Result<Response, ErrorResponse> {
            service_name = request.headers().get("Service-Name").cloned();
            Ok(response)
        };

    let ws_stream = tokio_tungstenite::accept_hdr_async(stream, header_callback)
        .await
        .unwrap();

    let (tx, rx) = unbounded();
    match &service_name {
        None => {
            println!("new user connection");
            if let Ok(events) = event_collection.clone().lock().await.find().await {
                events.iter().for_each(|event| {
                    let msg = Message::Binary(serde_json::to_vec(event).unwrap());
                    tx.unbounded_send(msg).unwrap();
                })
            }
            state.lock().await.insert(addr, tx);
        }
        Some(name) => {
            println!("service {} connection", name.to_str().unwrap())
        }
    }

    let (outgoing, incoming) = ws_stream.split();

    let service_name_ref = &service_name;
    let state_ref = &state.clone();
    let broadcast_incoming = incoming.try_for_each(|msg| async move {
        if let Some(name) = service_name_ref {
            println!("received message from {}", name.to_str()?);
            if let Ok(event_body) = serde_json::from_slice(msg.clone().into_data().as_slice()) {
                event_collection
                    .clone()
                    .lock()
                    .await
                    .save(event_body)
                    .await
                    .expect("cannot save event_body");
            }
            let peers = state_ref.lock().await;
            for recp in peers.iter() {
                recp.1.unbounded_send(msg.clone()).unwrap();
            }
        }

        Ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    state.lock().await.remove(&addr);
}
