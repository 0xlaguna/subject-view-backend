#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use rocket_cors::{AllowedOrigins, CorsOptions};
use std::str::FromStr;

pub mod routes;


#[tokio::main]
async fn start()  -> Result<(), rocket::Error> {
    let rocket = rocket::build();

    // Configure CORS
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: [
            "Get", "Put", "Post", "Delete", "Options", "Head", "Trace", "Connect", "Patch",
        ]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect(),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS.");

    routes::mount(rocket)
        .attach(Template::fairing())
        .manage(cors.clone())
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
