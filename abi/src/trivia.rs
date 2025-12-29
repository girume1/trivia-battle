use linera_sdk::linera_base_types::AccountOwner;
use serde::{Deserialize, Serialize};
use async_graphql::{InputObject, SimpleObject};

/// Question struct for the trivia game
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, PartialEq, Eq)]
pub struct Question {
    pub id: u64,
    pub text: String,
    pub choices: Vec<String>,
    pub correct_idx: u8,
    pub category: String,
    pub difficulty: u8,
}

/// Input version of Question for GraphQL mutations (AddQuestions)
#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct QuestionInput {
    pub text: String,
    pub choices: Vec<String>,
    pub correct_idx: u8,
    pub category: String,
    pub difficulty: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, PartialEq, Eq)]
pub struct TriviaGame {
    pub room_name: String,
    pub status: String,
    pub current_question_index: u8,
    pub players: Vec<PlayerScore>,
    pub pot: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, PartialEq, Eq)]
pub struct PlayerScore {
    pub player: AccountOwner,
    pub name: String,
    pub score: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, PartialEq, Eq)]
pub struct UserStatus {
    pub wins: u64,
    pub losses: u64,
    pub total_score: u64,
}