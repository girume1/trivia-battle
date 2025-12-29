use linera_sdk::linera_base_types::{AccountOwner, Amount, Timestamp};
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct TriviaBattle {
    pub room_name: String,
    pub owner: AccountOwner,
    pub max_players: u8,
    pub bet_amount: Amount,
    pub password: Option<String>,

    pub players: Vec<PlayerInBattle>,
    pub question_ids: Vec<u64>,
    pub full_questions: Vec<Question>, // From master
    pub current_question_index: u8,

    pub status: BattleStatus,
    pub pot: Amount,
    pub start_time: Option<Timestamp>,
    pub question_timeout_seconds: u64,  // 30
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PlayerInBattle {
    pub owner: AccountOwner,
    pub name: String,
    pub score: u64,
    pub has_answered_current: bool,
    pub last_answer_time: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
pub enum BattleStatus {
    Waiting,
    InProgress,
    Finished,
}

use abi::trivia::Question; // Make sure Question is in abi