mod minecraft;

use std::env;

use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use rocket::{routes, fs::FileServer, serde::json::{Json, serde_json::json, Value}, get, post};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Blast-off!");

    let _rocket = rocket::build()
        .mount("/", routes![team_leave, team_join, version_check, player_message])
        .launch()
        .await?;
    Ok(())

    // TODO pipe information to a discord bot

}

fn connect_to_db() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error Conencting to {}", database_url));
}

#[post("/teamjoin", data="<input>")]
fn team_join(input: Json<minecraft::Team>) {
    println!("{:?}", input);
}

#[post("/teamleave", data ="<input>")]
fn team_leave(input: Json<minecraft::Player>) {
    println!("{:?}", input);
}

#[post("/sentmessage", data ="<input>")]
// fn player_message(input: Json<minecraft::PlayerMsg>) {
fn player_message(input: Json<minecraft::PlayerMsg>) {
    println!("{:?}", input);
}
#[get("/version")]
fn version_check() -> rocket::serde::json::Value {
    json!({ "version": "1.0.0" })
}