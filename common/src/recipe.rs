use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub recipe: Recipe,
    pub created_at: DateTime<Utc>,
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
