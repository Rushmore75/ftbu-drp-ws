use rocket::{post, serde::json::{Json, serde_json::json}, get, State, response::stream::{EventStream, Event}, tokio::select, Shutdown};
use tokio::sync::broadcast::{Sender, error::RecvError};
use tracing::info;

use crate::{minecraft::{Team, Player, MinecraftMsg}, bot::bot_main};


#[post("/teamjoin", data="<input>")]
pub fn team_join(input: Json<Team>) {
    println!("{:?}", input);
}

#[post("/teamleave", data ="<input>")]
pub fn team_leave(input: Json<Player>) {
    println!("{:?}", input);
}
#[get("/version")]

pub fn version_check() -> rocket::serde::json::Value {
    json!({ "version": "1.0.0" })
}


// Minecraft -> Discord
#[post("/sentmessage", data ="<input>")]
pub async fn player_message(input: Json<MinecraftMsg>) {
    bot_main::send_msg_to_discord(&input.0).await;
}

// Discord -> Minecraft
#[get("/listenforchats")]
pub fn listen_for_chats(queue: &State<Sender<MinecraftMsg>>, mut end: Shutdown) -> EventStream![] {
    
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            // Messages get added to the queue from
            // the Discord message event.
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            
            // TODO say universe, etc
            info!("Sending \"{}\" to Minecraft.", msg.msg);
            // like return but doesn't exit
            yield Event::json(&msg);
        }
    }
}