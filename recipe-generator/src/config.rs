use serde::Deserialize;

use common::config::EventObserver;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_assets")]
    pub assets_source_path: String,
    pub kafka: Kafka,
    pub event_observer: EventObserver,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub recipe_generated_topic: String,
}

fn default_assets() -> String {
    "resources/assets.json".to_string()
}
