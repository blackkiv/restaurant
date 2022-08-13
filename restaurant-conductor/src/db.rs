use std::sync::Arc;

use futures::stream::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::{Bson, doc};
use mongodb::options::ClientOptions;
use serde_json::Value;
use tokio::sync::Mutex;

use common::model::Recipe;
use common::types::{EmptyResult, TypedResult};

use crate::config::Mongo;

#[derive(Debug)]
pub struct MongoCollections {
    pub recipe_collection: RecipeCollection,
}

#[derive(Debug)]
pub struct RecipeCollection {
    collection: Collection<Recipe>,
}

impl MongoCollections {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<Mutex<MongoCollections>> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let recipe_collection = db.collection::<Recipe>(&mongo_config.recipe_collection);

        Box::leak(Box::new(Arc::new(Mutex::new(MongoCollections {
            recipe_collection: RecipeCollection {
                collection: recipe_collection,
            },
        }))))
    }
}

impl RecipeCollection {
    pub async fn save(&self, recipe: Recipe) -> EmptyResult {
        let _ = &self.collection.insert_one(recipe, None).await?;
        Ok(())
    }

    pub async fn find_random(&self) -> TypedResult<Recipe> {
        let pipeline = doc! { "$sample": { "size": 1 } };
        self.collection
            .aggregate(vec![pipeline], None)
            .await?
            .try_next()
            .await?
            .ok_or_else(|| "empty collection".into())
            .map(|document| Value::from(Bson::from(document)))
            .map(|value| serde_json::from_value(value).unwrap())
    }
}
