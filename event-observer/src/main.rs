use common::config::load_config;

use crate::config::Config;
use crate::db::EventCollection;
use crate::server::SocketServer;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() {
    let config: &'static Config = load_config();
    let event_collection = EventCollection::load(&config.mongo).await;
    let server = SocketServer::create(config, event_collection).await;
    server.listen().await;
}
