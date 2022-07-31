use tokio::join;

use crate::config::Config;
use crate::listener::listen_events;

mod config;
mod listener;

#[tokio::main]
async fn main() {
    let config = Config::load();

    if let (Err(listener_error)) = join!(listen_events(&config)) {
        eprintln!("listener error {}", listener_error)
    }
}
