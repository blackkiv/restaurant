use std::error::Error;
use std::ops::Range;

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::config::Config;
use crate::ingredients;

#[derive(Deserialize, Debug)]
pub struct Ingredient {
    name: String,
    amount: u16,
}

pub struct Generator {
    ingredients: Vec<String>,
    range: Range<u16>,
}

impl Generator {
    pub fn init(config: &Config) -> Result<Generator, Box<dyn Error>> {
        let range = &config.generation_config.amount_range;
        Ok(Generator {
            ingredients: ingredients::available_ingredients(&config.ingredient_source_path)?,
            range: range.start..range.end,
        })
    }
}

impl Generator {
    pub fn generate_ingredient(&self) -> Ingredient {
        let ingredient = self.ingredients.choose(&mut thread_rng()).unwrap();
        let range = &self.range;

        Ingredient {
            name: ingredient.to_string(),
            amount: thread_rng().gen_range::<u16, Range<u16>>(range.clone()),
        }
    }
}
