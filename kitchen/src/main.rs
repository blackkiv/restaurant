#![feature(async_closure)]

use tokio::join;

use crate::config::Config;
use crate::db::MongoCollections;
use crate::listener::listen_events;

mod config;
mod db;
mod kitchen;
mod listener;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let collections = MongoCollections::load(&config.mongo).await;
    listen_events(config, collections).await;
}
