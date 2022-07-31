use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_ingredients")]
    pub ingredient_source_path: String,
    pub kafka: Kafka,
    pub generation_config: GenerationConfig,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub ingredient_generated_topic: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerationConfig {
    pub amount_range: AmountRange,
}

#[derive(Deserialize, Debug)]
pub struct AmountRange {
    pub start: u8,
    pub end: u8,
}

impl Config {
    pub fn load() -> Config {
        let config_source =
            fs::read_to_string("resources/config.toml").expect("config file not found");
        let config = toml::from_str(&config_source).expect("wrong config file format");
        dbg!(config)
    }
}

fn default_ingredients() -> String {
    "resources/ingredients.json".to_string()
}
