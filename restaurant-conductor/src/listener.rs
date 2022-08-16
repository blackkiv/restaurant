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
    let recipe_generated_listener_task = tokio::spawn(async move {
        recipe_generated_listener
            .subscribe(recipe_generated_consumer)
            .await
    });
    if let Err(error) = recipe_generated_listener_task.await {
        eprintln!("{}", error)
    }
}
