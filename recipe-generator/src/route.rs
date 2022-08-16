use rocket::post;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use common::config::{load_config, EventObserver};
use common::model::UserRecipe;
use common::types::TypedResult;
use common::KafkaProducer;

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
    let mut rcon_recipe_generated_producer = KafkaProducer::create(
        &kafka_config.host,
        &kafka_config.rcon_recipe_generated_topic,
        addr,
        service_name,
    )
    .await;
    let generator = Generator::init(config)?;
    let recipe = generator.generate_recipe(username).await;
    let user_recipe = UserRecipe {
        username: username.to_string(),
        recipe_hash: recipe.hash.to_string(),
    };
    let _ = recipe_generated_producer.send_message(&recipe).await;
    let _ = rcon_recipe_generated_producer
        .send_message(&user_recipe)
        .await;

    Ok(recipe.hash)
}
