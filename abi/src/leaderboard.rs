use linera_sdk::linera_base_types::{AccountOwner, Amount};
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

/// Single entry on the global leaderboard
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct LeaderboardEntry {
    /// Player's wallet address
    pub player: AccountOwner,

    /// Their display name
    pub name: String,

    /// Current tier
    pub tier: String,

    /// Total wins
    pub wins: u64,

    /// Win rate (e.g., 6500 = 65.00%)
    pub win_rate: u32,

    /// Lifetime winnings
    pub lifetime_winnings: Amount,

    /// Total score across all games
    pub total_score: u64,
}

/// Used when returning top 10 or paginated list
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
    pub updated_at: u64, // timestamp
}