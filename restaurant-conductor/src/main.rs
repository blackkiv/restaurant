#![feature(async_closure)]

use tokio::join;

use common::config::load_config;

use crate::config::Config;
use crate::db::MongoCollections;
use crate::generator::generate_order;
use crate::listener::listen_events;

mod config;
mod db;
mod generator;
mod listener;

#[tokio::main]
async fn main() {
    let config: &'static Config = load_config();
    let collection = MongoCollections::load(&config.mongo).await;
    if let (Err(listener_error), Err(generate_error)) = join!(
        tokio::spawn(listen_events(config, collection)),
        tokio::spawn(generate_order(config, collection))
    ) {
        eprintln!("{}{}", listener_error, generate_error);
    }
}
