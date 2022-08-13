use rocket::routes;

use config::Config;
use generator::Generator;

mod assets;
mod config;
mod generator;
mod route;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/api/v1", routes![route::generate_recipe])
        .launch()
        .await;
}
