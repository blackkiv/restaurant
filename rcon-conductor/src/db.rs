use std::sync::Arc;

use futures::stream::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use tokio::sync::Mutex;

use common::model::UserRecipe;
use common::types::{EmptyResult, TypedResult};

use crate::config::Mongo;

#[derive(Debug)]
pub struct UserRecipeCollection {
    collection: Collection<UserRecipe>,
}

impl UserRecipeCollection {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<Mutex<UserRecipeCollection>> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let user_recipe_collection =
            db.collection::<UserRecipe>(&mongo_config.user_recipe_collection);

        Box::leak(Box::new(Arc::new(Mutex::new(UserRecipeCollection {
            collection: user_recipe_collection,
        }))))
    }
}

impl UserRecipeCollection {
    pub async fn save(&self, user_recipe: UserRecipe) -> EmptyResult {
        let _ = &self.collection.insert_one(user_recipe, None).await?;
        Ok(())
    }

    pub async fn find_by_recipe_hash(&self, recipe_hash: String) -> TypedResult<UserRecipe> {
        let filter = doc! {"recipe_hash": {"$eq": recipe_hash}};
        self.collection
            .find(filter, None)
            .await?
            .try_next()
            .await?
            .ok_or_else(|| "empty collection".into())
    }
}
