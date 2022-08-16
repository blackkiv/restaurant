use serde::Deserialize;

use common::config::EventObserver;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka: Kafka,
    pub mongo: Mongo,
    pub generation_config: GenerationConfig,
    pub event_observer: EventObserver,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub consumer_group: String,
    pub recipe_generated_topic: String,
    pub order_prepared_topic: String,
    pub order_created_topic: String,
}

#[derive(Deserialize, Debug)]
pub struct Mongo {
    pub connection_url: String,
    pub database_name: String,
    pub recipe_collection: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerationConfig {
    pub interval: u64,
}
