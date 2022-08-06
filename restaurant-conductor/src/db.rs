use std::error::Error;
use std::sync::Arc;

use futures::stream::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::{Bson, doc, Document};
use mongodb::options::ClientOptions;
use serde_json::Value;
use tokio::sync::Mutex;

use common::recipe::Recipe;
use common::types::EmptyResult;

use crate::config::Mongo;

#[derive(Debug)]
pub struct RecipeCollection {
    collection: Collection<Recipe>,
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

    pub async fn find_random(&self) -> Result<Recipe, Box<dyn Error + Send + Sync>> {
        let pipeline = doc! { "$sample": { "size": 1 } };
        self
            .collection
            .aggregate(vec![pipeline], None)
            .await?
            .try_next()
            .await?
            .ok_or_else(|| "empty collection".into())
            .map(|document| Value::from(Bson::from(document)))
            .map(|value| serde_json::from_value(value).unwrap())
    }
}

impl Clone for RecipeCollection {
    fn clone(&self) -> Self {
        let collection = self.collection.clone_with_type::<Recipe>();
        RecipeCollection { collection }
    }
}
