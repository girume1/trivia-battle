use linera_sdk::linera_base_types::{Amount, ChainId};
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, PartialEq, Eq)]
pub struct PublicChainInfo {
    pub chain_id: ChainId,
    pub name: String,
    pub player_count: u32,
    pub active_rooms: u32,
    pub average_bet: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, PartialEq, Eq)]
pub struct RoomInfo {
    pub id: u64,
    pub chain_id: ChainId,
    pub name: String,
    pub current_players: u8,
    pub max_players: u8,
    pub bet_amount: Amount,
    pub has_password: bool,
    pub active: bool,
}