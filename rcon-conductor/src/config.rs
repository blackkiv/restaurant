use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka: Kafka,
    pub mongo: Mongo,
    pub rcon: Rcon,
}

#[derive(Deserialize, Debug)]
pub struct Kafka {
    pub host: String,
    pub consumer_group: String,
    pub rcon_recipe_generated_topic: String,
    pub order_prepared_topic: String,
}

#[derive(Deserialize, Debug)]
pub struct Mongo {
    pub connection_url: String,
    pub database_name: String,
    pub user_recipe_collection: String,
}

#[derive(Deserialize, Debug)]
pub struct Rcon {
    pub addr: String,
    pub password: String,
}
