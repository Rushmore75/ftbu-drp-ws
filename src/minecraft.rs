use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct Player {
    pub uuid: String,
    #[serde(rename(deserialize = "playerName"))]
    pub player_name: String,
    #[serde(rename(deserialize = "universeUuid"))]
    pub universe: String,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    pub owner: Player,
    pub id: String,
    #[serde(rename(deserialize = "universeUuid"))]
    pub universe: String,
    pub players: HashMap<Player, String>
}


#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct MinecraftMsg {
    #[serde(rename(deserialize = "player"))]
    #[serde(rename(serialize = "player"))]
    pub sender: Player,
    pub msg: String
}


impl Player {
    pub fn fake_player(name: String, universe: String) -> Self {
        Self {
            uuid: String::from(""),
            universe: universe,
            player_name: name,
        }
    }
}

impl MinecraftMsg {
    pub fn fake_message(author: String, message: String, universe: String) -> Self {
        Self {
            msg: message,
            sender: Player::fake_player(author, universe),
        }
    }
}