use std::error::Error;

use tokio::join;

use common::KafkaConsumer;
use common::recipe::Recipe;

use crate::Config;

pub async fn listen_events(config: &Config) -> Result<(), Box<dyn Error>> {
    let kafka_config = &config.kafka;
    let mut recipe_generated_consumer = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.recipe_generated_topic,
        &kafka_config.consumer_group,
    );
    let mut order_prepared_consumer = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.order_prepared_topic,
        &kafka_config.consumer_group,
    );

    if let (Err(recipe_error), Err(order_error)) = join!(
        recipe_generated_consumer.subscribe(&recipe_consumer),
        order_prepared_consumer.subscribe(&order_consumer)
    ) {
        eprintln!("recipe_generated error {}", recipe_error);
        eprintln!("order_prepared error {}", order_error);
    };

    Ok(())
}

fn recipe_consumer(row_event: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("recipe created event received");
    let recipe = serde_json::from_slice::<Recipe>(row_event)?;
    println!("{:#?}", recipe);
    Ok(())
}

fn order_consumer(row_event: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("order prepared event received");
    let recipe = serde_json::from_slice::<Recipe>(row_event)?;
    println!("{:#?}", recipe);
    Ok(())
}
