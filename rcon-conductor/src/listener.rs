use std::sync::Arc;

use tokio::join;
use tokio::sync::Mutex;

use common::KafkaConsumer;
use common::model::{Order, UserRecipe};
use common::types::EmptyResult;

use crate::config::Config;
use crate::db::UserRecipeCollection;
use crate::RconClient;

pub async fn listen_events(
    config: &Config,
    collection: &'static Arc<Mutex<UserRecipeCollection>>,
    rcon_client: &'static Arc<Mutex<RconClient>>,
) {
    let kafka_config = &config.kafka;
    let mut rcon_recipe_generated_listener = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.rcon_recipe_generated_topic,
        &kafka_config.consumer_group,
    );
    let mut order_prepared_listener = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.order_prepared_topic,
        &kafka_config.consumer_group,
    );
    let rcon_recipe_generated_consumer = async move |row_event: Vec<u8>| -> EmptyResult {
        let user_recipe = serde_json::from_slice::<UserRecipe>(row_event.as_slice())
            .map_err(|err| err.to_string())?;
        println!("recipe created event received {:?}", user_recipe);
        let user_recipe_username = user_recipe.username.to_string();
        collection
            .clone()
            .lock()
            .await
            .save(user_recipe)
            .await
            .map_err(|err| err.to_string())?;
        println!("recipe for user {} saved", user_recipe_username);
        Ok(())
    };
    let order_prepared_consumer = async move |row_event: Vec<u8>| -> EmptyResult {
        let order =
            serde_json::from_slice::<Order>(row_event.as_slice()).map_err(|err| err.to_string())?;
        println!("order prepared event received {:?}", order);
        let user_recipe = collection
            .clone()
            .lock()
            .await
            .find_by_recipe_hash(order.recipe.hash)
            .await?;
        rcon_client
            .clone()
            .lock()
            .await
            .give_reward(user_recipe.username);
        Ok(())
    };

    let rcon_recipe_generated_listener_task = tokio::spawn(async move {
        rcon_recipe_generated_listener
            .subscribe(rcon_recipe_generated_consumer)
            .await
    });
    let order_prepared_listener_task = tokio::spawn(async move {
        order_prepared_listener
            .subscribe(order_prepared_consumer)
            .await
    });

    if let (Err(rcon_recipe_generated_error), Err(order_prepared_error)) = join!(
        rcon_recipe_generated_listener_task,
        order_prepared_listener_task
    ) {
        eprintln!("{}, {}", rcon_recipe_generated_error, order_prepared_error);
    }
}
