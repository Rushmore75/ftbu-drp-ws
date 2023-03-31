mod minecraft;
mod config;
mod bot;
mod rest;
mod db;
mod schema;

use dotenvy::dotenv;
use tracing::debug;

#[tokio::main]
async fn main() {
    dotenv().ok();
    debug!("Starting...");
    
    // The rest api get's it's own thread
    std::thread::spawn(|| {
        let rocket = rest::rest_main::start_rocket();
        rocket.expect("Rocket Crashed");
    });

    // The discord bot can have this thread.
    let serenity = bot::bot_main::start_bot();
    serenity.await;
}
