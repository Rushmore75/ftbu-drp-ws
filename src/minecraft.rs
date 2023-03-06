use std::collections::HashMap;

use serde::Deserialize;


#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
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

#[derive(Deserialize, Debug)]
pub struct PlayerMsg {
    pub player: Player,
    pub msg: String
}