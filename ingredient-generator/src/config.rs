use serde::Deserialize;

use common::config::EventObserver;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_ingredients")]
    pub ingredient_source_path: String,
    pub kafka: Kafka,
    pub generation_config: GenerationConfig,
    pub event_observer: EventObserver,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub ingredient_generated_topic: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerationConfig {
    pub interval: u64,
    pub amount_range: AmountRange,
}

#[derive(Deserialize, Debug)]
pub struct AmountRange {
    pub start: u8,
    pub end: u8,
}

fn default_ingredients() -> String {
    "resources/ingredients.json".to_string()
}
