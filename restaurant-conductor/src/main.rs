#![feature(async_closure)]


use tokio::join;

use crate::config::Config;
use crate::db::RecipeCollection;
use crate::listener::listen_events;

mod config;
mod db;
mod listener;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let collection = RecipeCollection::load(&config.mongo).await;
    if let (Err(listener_error), ) = join!(tokio::spawn(listen_events(config, collection))) {
        eprintln!("{}", listener_error);
    }
}
