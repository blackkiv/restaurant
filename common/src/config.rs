use std::fmt::Debug;
use std::fs;

use serde::Deserialize;

pub fn load_config<T>() -> &'static T
where
    T: for<'a> Deserialize<'a> + Debug,
{
    let config_source = fs::read_to_string("resources/config.toml").expect("config file not found");
    let config = toml::from_str(&config_source).expect("wrong config file format");
    Box::leak(Box::new(dbg!(config)))
}
