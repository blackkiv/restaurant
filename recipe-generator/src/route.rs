// use rocket::{post, get};
use rocket::post;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use common::config::{EventObserver, load_config};
use common::KafkaProducer;
use common::types::TypedResult;

use crate::{Config, Generator};

#[derive(Deserialize, Serialize)]
pub struct GenerateRecipeRequest {
    username: String,
}

#[derive(Deserialize, Serialize)]
pub struct GenerateRecipeResponse {
    hash: String,
}

#[post("/generate-recipe", format = "application/json", data = "<request>")]
pub async fn generate_recipe(
    request: Json<GenerateRecipeRequest>,
) -> Result<Created<Json<GenerateRecipeResponse>>, NotFound<Json<String>>> {
    let username = &request.username;

    match handle_generation_request(username).await {
        Ok(recipe_hash) => Ok(Created::new("/generate-recipe")
            .body(Json::from(GenerateRecipeResponse { hash: recipe_hash }))),
        Err(error) => Err(NotFound(Json::from(error.to_string()))),
    }
}

async fn handle_generation_request(username: &str) -> TypedResult<String> {
    if username.is_empty() {
        return Err("username should not be empty".into());
    }
    let config: &'static Config = load_config();
    let kafka_config = &config.kafka;
    let EventObserver { addr, service_name } = &config.event_observer;
    let mut recipe_generated_producer = KafkaProducer::create(
        &kafka_config.host,
        &kafka_config.recipe_generated_topic,
        addr,
        service_name,
    )
        .await;
    let generator = Generator::init(config)?;
    let recipe = generator.generate_recipe(username).await;
    let _ = recipe_generated_producer.send_message(&recipe).await;

    Ok(recipe.hash)
}
