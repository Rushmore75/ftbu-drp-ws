use rocket::{post, serde::json::{Json, serde_json::json}, get, State, response::stream::{EventStream}, tokio::select, Shutdown};
use tokio::sync::broadcast::{Sender, error::RecvError};
use tracing::{info, debug};

use crate::{minecraft::{MinecraftMsg, PlayerUpdate, Event, EventType, player_name_or_unknown, team_name_or_unknown}, bot::bot_main};


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
    json!({ "version": "3.0.0" })
}


#[post("/sendevent/<universe>", data="<event>")]
pub async fn minecraft_event(event: Json<Event>, universe: String) {
    println!("{:?}", event);
    let message: String = match event.event_type {
        EventType::Rank=>{
            "rank change... TODO: this msg".to_owned()
        },
        EventType::TeamLeave => {
            let team_name = team_name_or_unknown(&event.sender); 
            let player_name = player_name_or_unknown(&event.sender); 
            format!("{} left {}", player_name, team_name)
        }, 
        EventType::TeamJoin => {
            let team_name = team_name_or_unknown(&event.sender); 
            let player_name = player_name_or_unknown(&event.sender); 
            format!("{} joined {}", player_name, team_name)
        },
        EventType::Disband => {
            let unknown = "An Unknown Team";

            let name = match &event.sender {
                Some(e) => {
                    match &e.team {
                        Some(t) => {
                            &t.id
                        },
                        None => {
                            unknown
                        },
                    }
                },
                None=> {
                    unknown
                },
            };

            format!("{} has disbanded!", unknown)
        }, 
        EventType::ServerStop => {
            "Server has stopped".to_owned()
        },
        EventType::ServerStart => {
            "Server has started".to_owned()
        },
        EventType::PlayerLogin => {
            let name = player_name_or_unknown(&event.sender);
            format!("{} has joined the game!", name)
        },
        EventType::PlayerLogout => {
            let name = player_name_or_unknown(&event.sender);
            format!("{} has left the game.", name)
        },
        EventType::PlayerDeath => {
            let player_name = player_name_or_unknown(&event.sender);
            let attacker_name = player_name_or_unknown(&event.receiver);

            format!("{} was killed by {}", player_name, attacker_name)
        },
    };
    bot_main::send_msg_to_discord(&MinecraftMsg::server_message(message, universe)).await;
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
            yield rocket::response::stream::Event::json(&msg);
        }
    }
}