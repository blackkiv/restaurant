use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mongo: Mongo,
    pub server: Server,
}

#[derive(Deserialize, Debug)]
pub struct Mongo {
    pub connection_url: String,
    pub database_name: String,
    pub event_collection: String,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub addr: String,
    pub port: u16,
}
