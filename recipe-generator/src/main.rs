use common::config::load_config;
use common::KafkaProducer;
use config::Config;
use generator::Generator;

mod assets;
mod config;
mod generator;

#[tokio::main]
async fn main() {
    let config: &'static Config = load_config();
    let kafka_config = &config.kafka;
    let mut recipe_generated_producer =
        KafkaProducer::create(&kafka_config.host, &kafka_config.recipe_generated_topic);
    if let Ok(generator) = Generator::init(config) {
        let recipe = generator.generate_recipe("blackkiv").await;
        if let Err(error) = recipe_generated_producer.send_message(&recipe).await {
            eprintln!("{:#?}", error);
        } else {
            println!("sent recipe: {:#?}", recipe);
        }
    }
}
