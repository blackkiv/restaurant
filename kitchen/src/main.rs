#![feature(async_closure)]

use common::config::load_config;

use crate::config::Config;
use crate::listener::listen_events;

mod config;
mod db;
mod kitchen;
mod listener;

#[tokio::main]
async fn main() {
    let config = load_config();
    listen_events(config).await;
}
