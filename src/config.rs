use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub universe_uuid: String, // This would also have to become a list for multi-server use
    pub guild_id: u64,
    pub relay_channel: u64,
}