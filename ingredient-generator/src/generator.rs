use std::ops::Range;

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

use common::model::Ingredient;
use common::types::TypedResult;

use crate::config::Config;
use crate::ingredients;

pub struct Generator {
    ingredients: Vec<String>,
    range: Range<u8>,
}

impl Generator {
    pub fn init(config: &Config) -> TypedResult<Generator> {
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
            amount: thread_rng().gen_range::<u8, Range<u8>>(range.clone()),
        }
    }
}
