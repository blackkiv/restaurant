use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

use crate::async_fn::AsyncFn;
use crate::types::EmptyStaticResult;

pub struct KafkaConsumer {
    consumer: Consumer,
}

impl KafkaConsumer {
    pub fn create(kafka_host: &str, topic: &str, group: &str) -> KafkaConsumer {
        let consumer = Consumer::from_hosts(vec![kafka_host.to_string()])
            .with_topic_partitions(topic.to_string(), &[0])
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(group.to_string())
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .expect("unable to create kafka consumer");

        KafkaConsumer { consumer }
    }
}

impl KafkaConsumer {
    pub async fn subscribe<Fun>(&mut self, consume_function: Fun) -> EmptyStaticResult
    where
        Fun: AsyncFn<Vec<u8>, Output = EmptyStaticResult> + Copy,
    {
        loop {
            for msg in self.consumer.poll().unwrap().iter() {
                for m in msg.messages() {
                    if let Err(error) = consume_function(m.value.to_vec()).await {
                        eprintln!("{}", error);
                        continue;
                    }
                }
                self.consumer.consume_messageset(msg)?;
            }
            self.consumer.commit_consumed()?;
        }
    }
}
