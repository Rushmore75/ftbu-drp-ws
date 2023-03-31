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
    pub player: Option<Player>,
    pub team: Option<Team>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    /// Who the event is happening to
    pub receiver: Option<PlayerUpdate>,
    /// Who caused the event 
    pub sender: Option<PlayerUpdate>,
    #[serde(rename(deserialize = "status"))]
    pub event_type: EventType,

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    // Teams
    Rank,
    #[serde(rename(deserialize = "LEAVE"))]
    TeamLeave,
    #[serde(rename(deserialize = "JOIN"))]
    TeamJoin,
    Disband,
    // Server
    #[serde(rename(deserialize = "SERVER_STOP"))]
    ServerStop,
    #[serde(rename(deserialize = "SERVER_START"))]
    ServerStart,
    // Player
    #[serde(rename(deserialize = "PLAYER_LOGIN"))]
    PlayerLogin,
    #[serde(rename(deserialize = "PLAYER_LOGOUT"))]
    PlayerLogout,
    #[serde(rename(deserialize = "PLAYER_DEATH"))]
    PlayerDeath,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct MinecraftMsg {
    #[serde(rename(deserialize = "player"))]
    #[serde(rename(serialize = "player"))]
    pub sender: Player,
    pub msg: String
}

pub fn team_name_or_unknown(team: &Option<PlayerUpdate>) -> &str {
     let unknown = "An Unknown Team";
    match &team {
        Some(t) => {
            match &t.team {
                Some(name) => {
                    &name.id
                },
                None => {
                    unknown
                },                    
            }
        },
        None => {
            unknown
        },
    }

}

pub fn player_name_or_unknown(player: &Option<PlayerUpdate>) -> &str {
    let unknown = "An Unknown Player";
    match &player {
        Some(sender) => {
            match &sender.player {
                Some(name) => {
                    &name.player_name
                },
                None => {
                    unknown
                },
            }
        },
        None => {
            unknown
        },
    }
}

impl PlayerUpdate {
    fn get_universe(&self) -> Option<&str> {
        match &self.player {
            Some(p) => {
                Some(&p.universe)
            },
            None => None,
        }
    }
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
    
    pub fn server_message(message: String, universe: String) -> Self {
        Self {
            msg: message,
            sender: Player::fake_player("".to_owned(), universe)
        }
    }
}