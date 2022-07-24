use std::error::Error;

use serde_json::Value;

use common::{KafkaConsumer, KafkaProducer};
use common::recipe::Recipe;
use config::Config;
use generator::Generator;

mod assets;
mod config;
mod generator;

fn main() {
    let config = Config::load();
    let kafka_config = &config.kafka;
    let mut generate_recipe_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.generate_recipe_topic);
    if let Ok(generator) = Generator::init(&config) {
        let recipe = generator.generate_recipe("blackkiv");
        if let Err(error) = generate_recipe_producer.send_message(&recipe) {
            eprintln!("{:#?}", error);
        } else {
            println!("sent recipe: {:#?}", recipe);
        }
    }
    println!("CONSUMING");
    let mut generate_recipe_consumer = KafkaConsumer::create(
        &kafka_config.host,
        &kafka_config.generate_recipe_topic,
        "recipe-generator",
    );
    if let Err(err) = generate_recipe_consumer.subscribe(&consume) {
        eprintln!("{}", err);
    }
}

fn consume(row_event: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("receive event");
    let recipe = serde_json::from_slice::<Recipe>(row_event)?;
    println!("{:#?}", recipe);
    Ok(())
}
