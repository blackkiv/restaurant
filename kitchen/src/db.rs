use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, FindOptions, UpdateOptions};

use common::model::{Ingredient, Order, OrderStatus};
use common::types::{EmptyResult, TypedResult};

use crate::config::Mongo;

pub struct OrderCollection {
    collection: Collection<Order>,
}

pub struct IngredientCollection {
    collection: Collection<Ingredient>,
}

impl OrderCollection {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<OrderCollection> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let order_collection = db.collection::<Order>(&mongo_config.order_collection);
        Box::leak(Box::new(Arc::new(OrderCollection {
            collection: order_collection,
        })))
    }
}

impl OrderCollection {
    pub async fn save(&self, order: Order) -> EmptyResult {
        let _ = &self.collection.insert_one(order, None).await?;
        Ok(())
    }

    pub async fn find_ordered_by_creation_date(&self) -> TypedResult<Vec<Order>> {
        let filter = doc! {"status": {"$eq": OrderStatus::CREATED.to_string()}};
        let sort = doc! {"created_at": 1};
        let options = FindOptions::builder().sort(sort).build();
        let orders = self
            .collection
            .find(filter, options)
            .await?
            .try_collect()
            .await?;
        Ok(orders)
    }

    pub async fn order_prepared(&self, order: &Order) -> EmptyResult {
        let query = doc! {"_id": order.id };
        let update = doc! {"$set": {"status" : OrderStatus::PREPARED.to_string()}};
        let _ = self.collection.update_one(query, update, None).await?;
        Ok(())
    }
}

impl IngredientCollection {
    pub async fn load(mongo_config: &Mongo) -> &'static Arc<IngredientCollection> {
        let client_options = ClientOptions::parse(&mongo_config.connection_url)
            .await
            .expect("unable to parse connection url");
        let client = Client::with_options(client_options).expect("unable to connect to mongodb");
        let db = client.database(&mongo_config.database_name);
        let ingredient_collection =
            db.collection::<Ingredient>(&mongo_config.ingredient_collection);
        Box::leak(Box::new(Arc::new(IngredientCollection {
            collection: ingredient_collection,
        })))
    }
}

impl IngredientCollection {
    pub async fn save(&self, ingredient: Ingredient, increase: bool) -> EmptyResult {
        let Ingredient { name, amount } = ingredient;
        let amount = amount as i32;
        let amount = if increase { amount } else { -amount };
        let query = doc! {"name": name};
        let update = doc! {"$inc": { "amount": amount }};
        let options = UpdateOptions::builder().upsert(true).build();
        let _ = &self.collection.update_one(query, update, options).await?;
        Ok(())
    }

    pub async fn find_all(&self) -> TypedResult<Vec<Ingredient>> {
        let ingredients = self
            .collection
            .find(None, None)
            .await?
            .try_collect()
            .await?;

        Ok(ingredients)
    }

    pub async fn remove_ingredients(&self, ingredients: &[Ingredient]) -> EmptyResult {
        for ingredient in ingredients {
            self.save(
                Ingredient {
                    name: ingredient.name.to_string(),
                    amount: ingredient.amount,
                },
                false,
            )
                .await?;
        }

        Ok(())
    }
}
