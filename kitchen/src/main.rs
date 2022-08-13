#![feature(async_closure)]

use crate::config::Config;
use crate::listener::listen_events;

mod config;
mod db;
mod kitchen;
mod listener;

#[tokio::main]
async fn main() {
    let config = Config::load();
    listen_events(config).await;
}
