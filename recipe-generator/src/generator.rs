use std::error::Error;

use sha2::{Digest, Sha256};

use common::recipe::{Ingredient, Recipe};

use crate::{assets, Config};
use crate::assets::Assets;

#[derive(Debug)]
pub struct Generator {
    assets: Assets,
}

impl Generator {
    pub fn init(config: &Config) -> Result<Generator, Box<dyn Error>> {
        Ok(Generator {
            assets: assets::load_assets(&config.assets_source_path)?,
        })
    }
}

/*

7EB080722FD6B1FFD038B5ABB5429A6CE72656CC2FA1D7015F8DBC4440596DB0

126 -- prefix
176 -- adjective1
128 -- adjective2
114 -- icon
47 214 | 177 255 | 208 56  \
181 171 | 181 66 | 154 108  |
231 38 | 86 204 | 47 161    |-- ingredients (amount, name)
215 1 | 95 141 | 188 68     |
64 89 | 109 176            /

*/
impl Generator {
    pub fn generate_recipe(&self, username: &str) -> Recipe {
        let (hexed, bytes) = hash_username(username);

        let mut ingredients = Vec::with_capacity(12);
        for i in (0..11u8).step_by(2) {
            let ingredient = self.resolve_ingredient(
                bytes[(3 + i) as usize],
                bytes[(3 + i + 1) as usize],
                i == 0,
            );
            if ingredient.amount != 0 {
                ingredients.push(ingredient)
            }
        }

        Recipe {
            hash: hexed,
            prefix: self.resolve_prefix(bytes[0]),
            adjective1: self.resolve_adjective(bytes[1]),
            adjective2: self.resolve_adjective(bytes[2]),
            icon: bytes[3],
            ingredients,
        }
    }

    fn resolve_prefix(&self, prefix: u8) -> String {
        let prefixes = &self.assets.prefixes;
        prefixes[(prefix % prefixes.len() as u8) as usize].to_string()
    }

    fn resolve_adjective(&self, adjective: u8) -> String {
        let adjectives = &self.assets.adjectives;
        adjectives[(adjective % adjectives.len() as u8) as usize].to_string()
    }

    fn resolve_ingredient(&self, amount_u: u8, name_u: u8, first: bool) -> Ingredient {
        let ingredients = &self.assets.ingredients;
        let name = ingredients[(name_u % ingredients.len() as u8) as usize].to_string();
        let amount = resolve_ingredient_amount(amount_u, first);

        fn resolve_ingredient_amount(amount_u: u8, first: bool) -> u8 {
            if first {
                amount_u % 10
            } else {
                match amount_u {
                    0..=100 => 0,
                    101..=170 => 1,
                    171..=254 => 2,
                    _ => 3,
                }
            }
        }

        Ingredient { name, amount }
    }
}

fn hash_username(username: &str) -> (String, Vec<u8>) {
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, username.as_bytes());
    let hash = hasher.finalize();
    let hex = format!("{:X}", hash);
    (hex, hash.to_vec())
}
