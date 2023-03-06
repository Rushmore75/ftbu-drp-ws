mod minecraft;
mod bot;
mod rest;

use std::env;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    println!("Starting...");
    dotenv().ok();
    
    
    // The rest api get's it's own thread
    std::thread::spawn(|| {
        let rocket = rest::interface::start_rocket();
        rocket.expect("Rocket Crashed");
    });

    // The discord bot can have this thread.
    let serenity = bot::bot_main::start_bot();
    serenity.await;
    
    // TODO pipe information to a discord bot

}

fn connect_to_db() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error Conencting to {}", database_url));
}
