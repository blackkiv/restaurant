use std::sync::Arc;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use tokio::sync::Mutex;

use common::config::EventObserver;
use common::KafkaProducer;
use common::model::{Order, OrderStatus};

use crate::config::Config;
use crate::MongoCollections;

pub async fn generate_order(
    config: &'static Config,
    collection: &'static Arc<Mutex<MongoCollections>>,
) {
    let kafka_config = &config.kafka;
    let EventObserver { addr, service_name } = &config.event_observer;
    let mut order_created_producer = KafkaProducer::create(
        &kafka_config.host,
        &kafka_config.order_created_topic,
        addr,
        service_name,
    )
        .await;
    let order_generator_task = tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(config.generation_config.interval));
            if let Ok(recipe) = collection
                .lock()
                .await
                .recipe_collection
                .find_random()
                .await
            {
                let order = Order {
                    id: ObjectId::new(),
                    recipe,
                    status: OrderStatus::Created,
                    created_at: Utc::now(),
                };
                match order_created_producer.send_message(&order).await {
                    Ok(_) => println!("successfully sent order_created event: {:?}", &order),
                    Err(error) => {
                        eprintln!("error while send order_created event {}", error);
                    }
                };
            }
        }
    });

    if let Err(error) = order_generator_task.await {
        eprintln!("{}", error)
    }
}
