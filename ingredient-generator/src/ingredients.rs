
use std::fs::File;
use std::io::BufReader;
use common::types::TypedResult;

pub fn available_ingredients(path: &str) -> TypedResult<Vec<String>> {
    let ingredients_source = File::open(path)?;
    let ingredients_reader = BufReader::new(ingredients_source);

    let ingredients: Vec<String> = serde_json::from_reader(ingredients_reader)?;
    Ok(ingredients)
}
