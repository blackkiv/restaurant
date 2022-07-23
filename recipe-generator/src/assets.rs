use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Assets {
    pub prefixes: Vec<String>,
    pub ingredients: Vec<String>,
    pub adjectives: Vec<String>,
}

pub fn load_assets(assets_source_path: &str) -> Result<Assets, Box<dyn Error>> {
    let assets_source = File::open(assets_source_path)?;
    let assets_reader = BufReader::new(assets_source);

    let assets = serde_json::from_reader(assets_reader)?;
    Ok(assets)
}
