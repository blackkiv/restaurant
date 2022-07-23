use std::thread;
use std::time::Duration;

use config::Config;
use generator::Generator;

mod config;
mod generator;
mod ingredients;

fn main() {
    let config = Config::load();
    if let Ok(generator) = Generator::init(&config) {
        loop {
            thread::sleep(Duration::from_secs(1));
            let ingredient = generator.generate_ingredient();
            println!("{:#?}", ingredient);
        }
    }
}
