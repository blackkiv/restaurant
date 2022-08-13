use std::fmt::{Display, Formatter};

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    CREATED,
    PREPARED,
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
