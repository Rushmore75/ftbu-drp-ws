use std::collections::HashMap;

use serde::Deserialize;


#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Player {
    uuid: String,
    #[serde(rename(deserialize = "playerName"))]
    player_name: String,
    #[serde(rename(deserialize = "universeUuid"))]
    universe: String,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    owner: Player,
    id: String,
    #[serde(rename(deserialize = "universeUuid"))]
    universe: String,
    players: HashMap<Player, String>
}

#[derive(Deserialize, Debug)]
pub struct PlayerMsg {
    player: Player,
    msg: String
}