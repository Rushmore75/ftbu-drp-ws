use rocket::{post, serde::json::{Json, serde_json::json}, get, State, response::stream::{EventStream, Event}, tokio::select, Shutdown};
use tokio::sync::broadcast::{Sender, error::RecvError};
use tracing::{info, debug};

use crate::{minecraft::{Team, Player, MinecraftMsg, PlayerUpdate}, bot::bot_main};


#[post("/updatePlayer", data="<input>")]
pub fn team_join(input: Json<PlayerUpdate>) {
    /*
    This will get called on when:
        Player joins team
        Player gets promoted / demoted
        Player creates (and joins) team 
     */
    println!("{:?}", input);
}

#[get("/version")]
pub fn version_check() -> rocket::serde::json::Value {
    // api version, if the mod wants to check.
    json!({ "version": "2.0.0" })
}

// Minecraft -> Discord
#[post("/sentmessage", data ="<input>")]
pub async fn player_message(input: Json<MinecraftMsg>) {
    bot_main::send_msg_to_discord(&input.0).await;
}

// Discord -> Minecraft
#[get("/listenforchats/<universe>")]
pub fn listen_for_chats(queue: &State<Sender<MinecraftMsg>>, universe: String, mut end: Shutdown) -> EventStream![] {
    
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            // Messages get added to the queue from
            // the Discord message event.
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => {
                        if msg.sender.universe == format!("\"{}\"", universe) {
                            debug!("Found correct universe.");
                            msg
                        } else {
                            debug!("Message for a different universe...");
                            continue
                        }
                    },
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            
            info!("Sending \"{}\" to Minecraft.", msg.msg);
            // like return but doesn't exit
            yield Event::json(&msg);
        }
    }
}