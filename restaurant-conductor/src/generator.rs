use std::sync::Arc;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use tokio::sync::Mutex;

use common::KafkaProducer;
use common::recipe::Order;

use crate::config::Config;
use crate::MongoCollections;

pub async fn generate_order(
    config: &'static Config,
    collection: &'static Arc<Mutex<MongoCollections>>,
) {
    let kafka_config = &config.kafka;
    let mut order_created_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.order_created_topic);
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
                    recipe,
                    created_at: Utc::now(),
                };
                match order_created_producer.send_message(&order).await {
                    Ok(_) => println!("successfully sent order_created event: {:?}", &order),
                    Err(error) => {
                        eprintln!("error while send ingredient_generated event {}", error);
                    }
                };
            }
        }
    });

    if let Err(error) = order_generator_task.await {
        eprintln!("{}", error)
    }
}
