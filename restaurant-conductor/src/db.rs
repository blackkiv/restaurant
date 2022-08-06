use std::error::Error;
use std::sync::Arc;

use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use tokio::sync::Mutex;

use common::recipe::Recipe;
use common::types::EmptyResult;

use crate::config::Mongo;

#[derive(Debug)]
pub struct RecipeCollection {
    pub collection: Collection<Recipe>,
}

impl RecipeCollection {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<Mutex<RecipeCollection>> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let recipe_collection = db.collection::<Recipe>(&mongo_config.recipe_collection);

        Box::leak(Box::new(Arc::new(Mutex::new(RecipeCollection {
            collection: recipe_collection,
        }))))
    }
}

impl RecipeCollection {
    pub async fn save(&self, recipe: Recipe) -> EmptyResult {
        let _ = &self.collection.insert_one(recipe, None).await?;
        Ok(())
    }
}

impl Clone for RecipeCollection {
    fn clone(&self) -> Self {
        let collection = self.collection.clone_with_type::<Recipe>();
        RecipeCollection { collection }
    }
}
