use std::sync::Arc;

use tokio::join;
use tokio::sync::Mutex;

use common::KafkaConsumer;
use common::model::{Order, Recipe};
use common::types::EmptyResult;

use crate::Config;
use crate::db::MongoCollections;

pub async fn listen_events(config: &Config, collection: &'static Arc<Mutex<MongoCollections>>) {
    let kafka_config = &config.kafka;
    let mut recipe_generated_listener = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.recipe_generated_topic,
        &kafka_config.consumer_group,
    );
    let mut order_prepared_listener = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.order_prepared_topic,
        &kafka_config.consumer_group,
    );
    let recipe_generated_consumer = async move |row_event: Vec<u8>| -> EmptyResult {
        let recipe = serde_json::from_slice::<Recipe>(row_event.as_slice())
            .map_err(|err| err.to_string())?;
        println!("recipe created event received {:?}", recipe);
        let recipe_hash = recipe.hash.clone();
        collection
            .lock()
            .await
            .recipe_collection
            .save(recipe)
            .await
            .map_err(|err| err.to_string())?;
        println!("recipe {} saved", recipe_hash);
        Ok(())
    };
    let order_prepared_consumer = async move |row_event: Vec<u8>| -> EmptyResult {
        let order =
            serde_json::from_slice::<Order>(row_event.as_slice()).map_err(|err| err.to_string())?;
        println!("order prepared event received {:?}", order);
        Ok(())
    };

    let recipe_generated_listener_task = tokio::spawn(async move {
        recipe_generated_listener
            .subscribe(recipe_generated_consumer)
            .await
    });
    let order_prepared_listener_task = tokio::spawn(async move {
        order_prepared_listener
            .subscribe(order_prepared_consumer)
            .await
    });

    if let (Err(recipe_generated_error), Err(order_prepared_error)) =
        join!(recipe_generated_listener_task, order_prepared_listener_task)
    {
        eprintln!("{}, {}", recipe_generated_error, order_prepared_error);
    }
}
