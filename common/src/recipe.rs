use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Recipe {
    pub hash: String,
    pub prefix: String,
    pub adjective1: String,
    pub adjective2: String,
    pub icon: u8,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Deserialize, Debug)]
pub struct Ingredient {
    pub name: String,
    pub amount: u8,
}
