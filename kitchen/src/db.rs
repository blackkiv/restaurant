use std::error::Error;
use std::sync::Arc;

use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, UpdateOptions};
use tokio::sync::Mutex;

use common::recipe::{Ingredient, Order};
use common::types::EmptyResult;

use crate::config::Mongo;

pub struct MongoCollections {
    pub order_collection: OrderCollection,
    pub ingredient_collection: IngredientCollection,
}

pub struct OrderCollection {
    collection: Collection<Order>,
}

pub struct IngredientCollection {
    collection: Collection<Ingredient>,
}

impl MongoCollections {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<Mutex<MongoCollections>> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let order_collection = db.collection::<Order>(&mongo_config.order_collection);
        let ingredient_collection =
            db.collection::<Ingredient>(&mongo_config.ingredient_collection);

        Box::leak(Box::new(Arc::new(Mutex::new(MongoCollections {
            order_collection: OrderCollection {
                collection: order_collection,
            },
            ingredient_collection: IngredientCollection {
                collection: ingredient_collection,
            },
        }))))
    }
}

impl OrderCollection {
    pub async fn save(&self, order: Order) -> EmptyResult {
        let _ = &self.collection.insert_one(order, None).await?;
        Ok(())
    }
    pub async fn find_ordered_by_creation_date(&self) -> Result<&[Order], Box<dyn Error>> {
        todo!()
    }
}

impl IngredientCollection {
    pub async fn save(&self, ingredient: Ingredient) -> EmptyResult {
        let Ingredient { name, amount } = ingredient;
        let amount = amount as i32;
        let query = doc! {"name": name};
        let update = doc! {"$inc": { "amount": amount }};
        let options = UpdateOptions::builder().upsert(true).build();
        let _ = &self.collection.update_one(query, update, options).await?;
        Ok(())
    }
}
