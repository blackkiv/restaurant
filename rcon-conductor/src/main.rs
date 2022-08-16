#![feature(async_closure)]

use common::config::load_config;
use listener::listen_events;

use crate::config::Config;
use crate::db::UserRecipeCollection;
use crate::rcon::RconClient;

mod config;
mod db;
mod listener;
mod rcon;

#[tokio::main]
async fn main() {
    let config: &'static Config = load_config();
    let rcon_client = RconClient::create(&config.rcon);
    let user_recipe_collection = UserRecipeCollection::load(&config.mongo).await;
    listen_events(config, user_recipe_collection, rcon_client).await;
}
