use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn available_ingredients(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let ingredients_source = File::open(path)?;
    let ingredients_reader = BufReader::new(ingredients_source);

    let ingredients: Vec<String> = serde_json::from_reader(ingredients_reader)?;
    Ok(ingredients)
}
