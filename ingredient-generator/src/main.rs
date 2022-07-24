use std::error::Error;
use std::thread;
use std::time::Duration;

use common::KafkaProducer;
use config::Config;
use generator::Generator;

mod config;
mod generator;
mod ingredients;

fn main() {
    let config = Config::load();
    let kafka_config = &config.kafka;
    let mut generate_ingredient_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.generate_ingredient_topic);
    if let Ok(generator) = Generator::init(&config) {
        loop {
            thread::sleep(Duration::from_secs(1));
            let ingredient = generator.generate_ingredient();
            match generate_ingredient_producer.send_message(&ingredient) {
                Ok(_) => println!(
                    "successfully sent generate_ingredient event: {:?}",
                    &ingredient
                ),
                Err(error) => eprintln!("error while send generate_ingredient event {}", error),
            }
        }
    }
}
