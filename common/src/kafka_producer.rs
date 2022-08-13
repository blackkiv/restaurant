use std::time::Duration;

use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};
use serde::Serialize;

use crate::types::EmptyStaticResult;

pub struct KafkaProducer {
    producer: Producer,
    topic: String,
}

impl KafkaProducer {
    pub fn create(kafka_host: &str, topic: &str) -> KafkaProducer {
        let producer = Producer::from_hosts(vec![kafka_host.to_string()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .expect("unable to create kafka producer");

        KafkaProducer {
            producer,
            topic: topic.to_string(),
        }
    }
}

impl KafkaProducer {
    pub async fn send_message<T: Serialize>(&mut self, object: &T) -> EmptyStaticResult {
        let json = serde_json::to_vec(object)?;
        let _ = &self.producer.send(&Record::from_value(&self.topic, json))?;
        Ok(())
    }
}
