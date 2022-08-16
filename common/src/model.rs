use std::fmt::{Debug, Display, Formatter};

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub emitted_at: DateTime<Utc>,
    pub body: EventBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventBody {
    pub body_type: EventBodyType,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventBodyType {
    Order,
    Recipe,
    Ingredient,
    Message,
}

impl From<&Order> for EventBodyType {
    fn from(_: &Order) -> Self {
        EventBodyType::Order
    }
}

impl From<&Recipe> for EventBodyType {
    fn from(_: &Recipe) -> Self {
        EventBodyType::Recipe
    }
}

impl From<&Ingredient> for EventBodyType {
    fn from(_: &Ingredient) -> Self {
        EventBodyType::Ingredient
    }
}

impl From<&String> for EventBodyType {
    fn from(_: &String) -> Self {
        EventBodyType::Message
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub recipe: Recipe,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
    Created,
    Prepared,
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub hash: String,
    pub prefix: String,
    pub adjective1: String,
    pub adjective2: String,
    pub icon: u8,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub name: String,
    pub amount: u8,
}
