use tokio::join;

use common::model::{Ingredient, Order};
use common::types::EmptyResult;
use common::KafkaConsumer;

use crate::db::{IngredientCollection, OrderCollection};
use crate::kitchen::Kitchen;
use crate::Config;

pub async fn listen_events(config: &Config) {
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
    let order_collection = OrderCollection::load(&config.mongo).await;
    let ingredient_collection = IngredientCollection::load(&config.mongo).await;
    let kitchen = Kitchen::create(config, order_collection, ingredient_collection).await;

    let order_created_consumer = async move |row_event: Vec<u8>| -> EmptyResult {
        let order =
            serde_json::from_slice::<Order>(row_event.as_slice()).map_err(|err| err.to_string())?;
        println!("order created event received {:?}", order);
        order_collection.save(order).await?;
        println!("order saved");
        kitchen.lock().await.try_cook_orders().await?;
        Ok(())
    };
    let ingredient_generated_consumer = async move |row_event: Vec<u8>| -> EmptyResult {
        let ingredient = serde_json::from_slice::<Ingredient>(row_event.as_slice())
            .map_err(|err| err.to_string())?;
        println!("ingredient generated event received {:?}", ingredient);
        ingredient_collection.save(ingredient, true).await?;
        println!("ingredient saved");
        kitchen.lock().await.try_cook_orders().await?;
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
