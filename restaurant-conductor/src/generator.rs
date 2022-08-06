use std::sync::Arc;
use std::thread;
use std::time::Duration;

use tokio::sync::Mutex;

use common::KafkaProducer;

use crate::config::Config;
use crate::RecipeCollection;

pub async fn generate_order(
    config: &'static Config,
    collection: &'static Arc<Mutex<RecipeCollection>>,
) {
    let kafka_config = &config.kafka;
    let mut order_created_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.order_created_topic);
    let order_generator_task = tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(config.generation_config.interval));
            if let Ok(recipe) = collection.lock().await.find_random().await {
                match order_created_producer.send_message(&recipe).await {
                    Ok(_) => println!("successfully sent ingrorder_created event: {:?}", &recipe),
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
