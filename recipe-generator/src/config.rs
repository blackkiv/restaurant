use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_assets")]
    pub assets_source_path: String,
    pub kafka: Kafka,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub recipe_generated_topic: String,
}

impl Config {
    pub fn load() -> Config {
        let config_source =
            fs::read_to_string("resources/config.toml").expect("config file not found");
        let config = toml::from_str(&config_source).expect("wrong config file format");
        dbg!(config)
    }
}

fn default_assets() -> String {
    "resources/assets.json".to_string()
}
