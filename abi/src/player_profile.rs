use linera_sdk::linera_base_types::Amount;
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

/// Player's personal profile — shown in lobby, game, leaderboard
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Default)]
pub struct PlayerProfile {
    /// Display name chosen by player (or wallet address fallback)
    pub display_name: String,

    /// Current tier (e.g., "Bronze", "Silver", "Gold", "Diamond")
    pub tier: String,

    /// Minimum bet this player can place (based on tier)
    pub min_bet_allowed: Amount,

    /// Reward multiplier on wins (e.g., 100 = 1x, 150 = 1.5x)
    pub reward_multiplier: u32,

    /// Total games played
    pub games_played: u64,

    /// Total wins
    pub wins: u64,

    /// Lifetime winnings in tokens
    pub lifetime_winnings: Amount,

    /// Total points scored across all games
    pub total_score: u64,
}