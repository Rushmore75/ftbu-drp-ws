use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct Player {
    pub uuid: String,
    #[serde(rename(deserialize = "playerName"))]
    #[serde(rename(serialize = "playerName"))]
    pub player_name: String,
    #[serde(rename(deserialize = "universeUuid"))]
    #[serde(rename(serialize = "universeUuid"))]
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

#[derive(Deserialize, Debug)]
pub struct PlayerUpdate {
    #[serde(default = "default_player")]
    pub player: Player,
    pub team: Team,
    pub status: String
}

fn default_player() -> Player {
    Player { uuid: "".to_owned(), player_name: "".to_owned(), universe: "".to_owned() }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Rank,
    Leave,
    Join,
    Disband
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