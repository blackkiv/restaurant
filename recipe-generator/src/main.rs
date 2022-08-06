use std::error::Error;

use common::{KafkaConsumer, KafkaProducer};
use common::recipe::Recipe;
use common::types::EmptyResult;
use config::Config;
use generator::Generator;

mod assets;
mod config;
mod generator;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let kafka_config = &config.kafka;
    let mut recipe_generated_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.recipe_generated_topic);
    if let Ok(generator) = Generator::init(&config) {
        let recipe = generator.generate_recipe("blackkiv").await;
        if let Err(error) = recipe_generated_producer.send_message(&recipe).await {
            eprintln!("{:#?}", error);
        } else {
            println!("sent recipe: {:#?}", recipe);
        }
    }
}
