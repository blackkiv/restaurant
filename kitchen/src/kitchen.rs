use std::sync::Arc;

use tokio::sync::Mutex;

use common::KafkaProducer;
use common::types::EmptyStaticResult;

use crate::config::Kafka;
use crate::MongoCollections;

pub struct Kitchen {
    collections: &'static Arc<Mutex<MongoCollections>>,
    order_prepared_producer: KafkaProducer,
}

impl Kitchen {
    pub fn create(
        kafka_config: &Kafka,
        collections: &'static Arc<Mutex<MongoCollections>>,
    ) -> &'static Arc<Kitchen> {
        let producer =
            KafkaProducer::create(&kafka_config.host, &kafka_config.order_prepared_topic);

        Box::leak(Box::new(Arc::new(Kitchen {
            collections,
            order_prepared_producer: producer,
        })))
    }
}

impl Kitchen {
    pub async fn try_cook_orders(&self) -> EmptyStaticResult {
        todo!()
    }
}
