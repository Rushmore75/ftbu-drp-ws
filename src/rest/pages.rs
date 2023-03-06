use rocket::{post, serde::json::{Json, serde_json::json}, get};

use crate::{minecraft::{Team, Player, PlayerMsg}, bot::{self, bot_main}};


#[post("/teamjoin", data="<input>")]
pub fn team_join(input: Json<Team>) {
    println!("{:?}", input);
}

#[post("/teamleave", data ="<input>")]
pub fn team_leave(input: Json<Player>) {
    println!("{:?}", input);
}

#[post("/sentmessage", data ="<input>")]
// fn player_message(input: Json<minecraft::PlayerMsg>) {
pub async fn player_message(input: Json<PlayerMsg>) {
    println!("{:?}", input);
    // TODO later we will get his message from a command
    // such as `/place-chat-here` or something
    bot_main::send_msg_to_discord(input.0).await;
}
#[get("/version")]
pub fn version_check() -> rocket::serde::json::Value {
    json!({ "version": "1.0.0" })
}