use std::error::Error;
use std::sync::Arc;

use minecraft_client_rs::{Client, Message};
use tokio::sync::Mutex;

use crate::config::Rcon;

pub struct RconClient {
    client: Client,
}

impl RconClient {
    pub fn create(rcon_config: &Rcon) -> &'static Arc<Mutex<RconClient>> {
        let mut client =
            Client::new(rcon_config.addr.to_string()).expect("unable to connect to server");

        let _ = client
            .authenticate(rcon_config.password.to_string())
            .expect("unable to authenticate client");

        Box::leak(Box::new(Arc::new(Mutex::new(RconClient { client }))))
    }
}

impl RconClient {
    pub fn give_reward(&mut self, username: String) {
        match self
            .client
            .send_command(format!("give {} minecraft:diamond 10", username))
        {
            Ok(msg) => {
                println!("{:#?}", msg)
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }
}

impl Drop for RconClient {
    fn drop(&mut self) {
        self.client.close().unwrap();
    }
}
