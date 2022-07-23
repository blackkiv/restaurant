use std::error::Error;

use config::Config;
use generator::Generator;

mod assets;
mod config;
mod generator;

fn main() {
    let config = Config::load();
    // Generator::init(&config).unwrap();
    if let Ok(generator) = Generator::init(&config) {
        let recipe = generator.generate_recipe("blackkiv");
        println!("{:#?}", recipe)
    }
}
