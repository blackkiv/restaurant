use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka: Kafka,
    pub mongo: Mongo,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub consumer_group: String,
    pub order_prepared_topic: String,
    pub order_created_topic: String,
    pub ingredient_generated_topic: String,
}

#[derive(Deserialize, Debug)]
pub struct Mongo {
    pub connection_url: String,
    pub database_name: String,
    pub order_collection: String,
    pub ingredient_collection: String,
}

impl Config {
    pub fn load() -> &'static Config {
        let config_source =
            fs::read_to_string("resources/config.toml").expect("config file not found");
        let config = toml::from_str(&config_source).expect("wrong config file format");
        Box::leak(Box::new(dbg!(config)))
    }
}
