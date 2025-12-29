use linera_sdk::linera_base_types::AccountOwner;
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

/// Lightweight stats — used in game room player list
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PlayerStats {
    /// Player address
    pub owner: AccountOwner,

    /// Current game streak
    pub current_streak: u32,

    /// Best streak ever
    pub best_streak: u32,

    /// Average answer time in ms
    pub avg_answer_time_ms: u32,

    /// Total games played today
    pub games_today: u32,

    /// Daily bonus claimed today?
    pub daily_bonus_claimed: bool,
}