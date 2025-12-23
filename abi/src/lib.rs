use linera_sdk::base::{AccountOwner, ChainId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub owner: AccountOwner,
    pub balance: u64,           // Current token balance
    pub score: u32,             // Cumulative score across rounds
    pub bet_amount: u64,        // Bet placed in the current round (0 if none)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
    pub correct_index: usize,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct GameState {
    pub room_id: String,
    pub players: Vec<Player>,
    pub current_question: Option<Question>,
    pub answers: Vec<(ChainId, usize)>, // (player chain id, chosen index)
    pub round_active: bool,
    pub pot: u64,                       // Total tokens bet in the current round
}
