use std::time::Duration;

use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures_util::StreamExt;
use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};
use serde::Serialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::{generate_key, Request};
use tokio_tungstenite::tungstenite::http::Uri;
use tokio_tungstenite::tungstenite::Message;

use crate::model::{EventBody, EventBodyType};
use crate::types::{EmptyResult, TypedResult};

pub struct KafkaProducer {
    producer: Producer,
    event_observer: UnboundedSender<Message>,
    topic: String,
}

impl KafkaProducer {
    pub async fn create(
        kafka_host: &str,
        topic: &str,
        observer_url: &str,
        service_name: &str,
    ) -> KafkaProducer {
        let producer = Producer::from_hosts(vec![kafka_host.to_string()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .expect("unable to create kafka producer");

        let event_observer = create_event_observer_client(observer_url, service_name).await;

        KafkaProducer {
            producer,
            event_observer,
            topic: topic.to_string(),
        }
    }
}

async fn create_event_observer_client(
    observer_url: &str,
    service_name: &str,
) -> UnboundedSender<Message> {
    let (tx, rx) = unbounded();
    tokio::spawn(forward_event(
        rx,
        observer_url.to_string(),
        service_name.to_string(),
    ));
    tx
}

async fn forward_event(rx: UnboundedReceiver<Message>, observer_url: String, service_name: String) {
    let client_request = create_client_request(&observer_url, &service_name)
        .expect("unable to create client request");
    let (ws_stream, _) = connect_async(client_request)
        .await
        .expect("failed to connect");
    println!("connected to event observer");

    let (write, _) = ws_stream.split();

    let ch_to_ws = rx.map(Ok).forward(write);

    ch_to_ws.await.expect("unable to send event");
}

fn create_client_request(url: &str, service_name: &str) -> TypedResult<Request> {
    Ok(Request::builder()
        .method("GET")
        .header("Host", url)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .header("Service-Name", service_name)
        .uri(url.parse::<Uri>()?)
        .body(())?)
}

impl KafkaProducer {
    pub async fn send_message<T: Serialize>(&mut self, object: &T) -> EmptyResult
    where
        EventBodyType: for<'a> From<&'a T>,
    {
        let json_object = serde_json::to_vec(&object)?;
        let _ = &self
            .producer
            .send(&Record::from_value(&self.topic, json_object.clone()))?;
        let event = EventBody {
            body_type: object.into(),
            body: json_object,
        };
        if matches!(event.body_type, EventBodyType::Ignore) {
            let event_json = serde_json::to_vec(&event)?;
            self.event_observer
                .unbounded_send(Message::Binary(event_json))?;
        }
        Ok(())
    }
}
