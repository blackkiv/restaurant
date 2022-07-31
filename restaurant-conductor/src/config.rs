use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka: Kafka,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub consumer_group: String,
    pub recipe_generated_topic: String,
    pub order_prepared_topic: String,
    pub order_created_topic: String,
}

impl Config {
    pub fn load() -> Config {
        let config_source =
            fs::read_to_string("resources/config.toml").expect("config file not found");
        let config = toml::from_str(&config_source).expect("wrong config file format");
        dbg!(config)
    }
}
