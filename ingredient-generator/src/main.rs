use std::thread;
use std::time::Duration;
use common::config::load_config;

use common::KafkaProducer;
use config::Config;
use generator::Generator;

mod config;
mod generator;
mod ingredients;

#[tokio::main]
async fn main() {
    let config: &'static Config = load_config();
    let kafka_config = &config.kafka;
    let mut ingredient_generated_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.ingredient_generated_topic);
    if let Ok(generator) = Generator::init(config) {
        loop {
            thread::sleep(Duration::from_secs(config.generation_config.interval));
            let ingredient = generator.generate_ingredient();
            match ingredient_generated_producer
                .send_message(&ingredient)
                .await
            {
                Ok(_) => println!(
                    "successfully sent ingredient_generated event: {:?}",
                    &ingredient
                ),
                Err(error) => eprintln!("error while send ingredient_generated event {}", error),
            }
        }
    }
}
