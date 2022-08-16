use std::sync::Arc;

use tokio::sync::Mutex;

use common::config::EventObserver;
use common::model::{Ingredient, Order};
use common::types::EmptyResult;
use common::KafkaProducer;

use crate::db::{IngredientCollection, OrderCollection};
use crate::Config;

pub struct Kitchen {
    order_collection: &'static Arc<OrderCollection>,
    ingredient_collection: &'static Arc<IngredientCollection>,
    order_prepared_producer: KafkaProducer,
}

impl Kitchen {
    pub async fn create(
        config: &Config,
        order_collection: &'static Arc<OrderCollection>,
        ingredient_collection: &'static Arc<IngredientCollection>,
    ) -> &'static Arc<Mutex<Kitchen>> {
        let kafka_config = &config.kafka;
        let EventObserver { addr, service_name } = &config.event_observer;
        let producer = KafkaProducer::create(
            &kafka_config.host,
            &kafka_config.order_prepared_topic,
            addr,
            service_name,
        )
        .await;

        Box::leak(Box::new(Arc::new(Mutex::new(Kitchen {
            order_collection,
            ingredient_collection,
            order_prepared_producer: producer,
        }))))
    }
}

impl Kitchen {
    pub async fn try_cook_orders(&mut self) -> EmptyResult {
        let orders = self
            .order_collection
            .find_ordered_by_creation_date()
            .await?;

        let available_ingredients = self.ingredient_collection.find_all().await?;

        println!("try to cook orders");
        let orders = orders.iter().filter(|order| {
            let recipe_ingredients = &order.recipe.ingredients;
            enough_ingredients(
                recipe_ingredients.as_slice(),
                available_ingredients.as_slice(),
            )
        });
        let orders_count = orders.clone().count();
        if orders_count == 0 {
            println!("no orders to cook");
        } else {
            println!("{} orders can be cooked", orders_count);
        }
        for order in orders {
            self.prepare_order(order).await?;
        }
        Ok(())
    }

    async fn prepare_order(&mut self, order: &Order) -> EmptyResult {
        println!("prepare order {} with hash {}", order.id, order.recipe.hash);
        self.order_prepared_producer.send_message(order).await?;
        self.order_collection.order_prepared(order).await?;
        let ingredients = &order.recipe.ingredients;
        self.ingredient_collection
            .remove_ingredients(ingredients.as_slice())
            .await?;
        println!(
            "order {} with hash {} prepared",
            order.id, order.recipe.hash
        );
        Ok(())
    }
}

fn enough_ingredients(recipe: &[Ingredient], available: &[Ingredient]) -> bool {
    recipe
        .iter()
        .map(|recipe_ingredient| {
            available.iter().any(|available_ingredient| {
                available_ingredient.name == recipe_ingredient.name
                    && available_ingredient.amount >= recipe_ingredient.amount
            })
        })
        .all(|availability| availability)
}
