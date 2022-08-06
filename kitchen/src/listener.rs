use std::sync::Arc;

use tokio::join;
use tokio::sync::Mutex;

use common::KafkaConsumer;
use common::recipe::{Ingredient, Order};
use common::types::EmptyStaticResult;

use crate::{Config, MongoCollections};
use crate::kitchen::Kitchen;

pub async fn listen_events(config: &Config, collection: &'static Arc<Mutex<MongoCollections>>) {
    let kafka_config = &config.kafka;
    let mut order_created_listener = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.order_created_topic,
        &kafka_config.consumer_group,
    );
    let mut ingredient_generated_listener = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.ingredient_generated_topic,
        &kafka_config.consumer_group,
    );
    let kitchen = Kitchen::create(kafka_config, collection);

    let order_created_consumer = async move |row_event: Vec<u8>| -> EmptyStaticResult {
        let order =
            serde_json::from_slice::<Order>(row_event.as_slice()).map_err(|err| err.to_string())?;
        println!("order created event received {:?}", order);
        collection
            .lock()
            .await
            .order_collection
            .save(order)
            .await
            .map_err(|err| err.to_string())?;
        println!("order saved");
        kitchen.try_cook_orders().await?;
        Ok(())
    };
    let ingredient_generated_consumer = async move |row_event: Vec<u8>| -> EmptyStaticResult {
        let ingredient = serde_json::from_slice::<Ingredient>(row_event.as_slice())
            .map_err(|err| err.to_string())?;
        println!("ingredient generated event received {:?}", ingredient);
        collection
            .lock()
            .await
            .ingredient_collection
            .save(ingredient)
            .await
            .map_err(|err| err.to_string())?;
        println!("ingredient saved");
        kitchen.try_cook_orders().await?;
        Ok(())
    };

    let order_created_listener_task = tokio::spawn(async move {
        order_created_listener
            .subscribe(order_created_consumer)
            .await
    });
    let ingredient_generated_listener_task = tokio::spawn(async move {
        ingredient_generated_listener
            .subscribe(ingredient_generated_consumer)
            .await
    });

    if let (Err(order_created_error), Err(ingredient_generated_error)) = join!(
        order_created_listener_task,
        ingredient_generated_listener_task
    ) {
        eprintln!("{}, {}", order_created_error, ingredient_generated_error);
    }
}
