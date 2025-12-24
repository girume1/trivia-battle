use linera_sdk::{
    base::{AccountOwner, ChainId},
    Contract, ContractRuntime, WithContractRuntime,
};
use serde::{Deserialize, Serialize};
use crate::{GameState, Player, Question};

#[derive(Serialize, Deserialize)]
pub enum Operation {
    CreateRoom { room_id: String },
    JoinRoom { room_id: String },
    StartRoundWithBet { bet_amount: u64 },  // New: Deduct bet and start round
    SubmitAnswer { answer_index: usize },
    EndRound,  // New: Resolve answers, pay winners, end round
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    // For future cross-chain communication
}

pub struct TriviaContract {
    runtime: ContractRuntime<Self>,
    state: GameState,
}

impl WithContractRuntime for TriviaContract {
    fn runtime(&self) -> &ContractRuntime<Self> {
        &self.runtime
    }

    fn runtime_mut(&mut self) -> &mut ContractRuntime<Self> {
        &mut self.runtime
    }
}

impl Contract for TriviaContract {
    type Message = Message;
    type Parameters = ();
    type ApplicationCall = Operation;

    fn new(runtime: ContractRuntime<Self>) -> Self {
        Self {
            runtime,
            state: GameState::default(),
        }
    }

    async fn initialize(&mut self, _parameters: (), call: Operation) -> Result<(), String> {
        match call {
            Operation::CreateRoom { room_id } => {
                if !self.state.room_id.is_empty() {
                    return Err("Room already initialized".to_string());
                }
                self.state.room_id = room_id;
                Ok(())
            }
            _ => Err("Only CreateRoom is allowed during initialization".to_string()),
        }
    }

    async fn execute_operation(&mut self, operation: Operation) -> Result<(), String> {
        match operation {
            Operation::CreateRoom { .. } => Err("Room already created".to_string()),

            Operation::JoinRoom { room_id } => {
                if self.state.room_id != room_id {
                    return Err(format!("Wrong room ID. Expected: {}", self.state.room_id));
                }
                let owner = self.runtime.application_owner();
                if self.state.players.iter().any(|p| p.owner == owner) {
                    return Err("Player already joined".to_string());
                }
                self.state.players.push(Player {
                    owner,
                    balance: 1000, // Starting balance for betting
                    score: 0,
                });
                Ok(())
            }

            Operation::StartRoundWithBet { bet_amount } => {
                if self.state.round_active {
                    return Err("Round already active".to_string());
                }
                if bet_amount == 0 {
                    return Err("Bet amount must be greater than 0".to_string());
                }

                let owner = self.runtime.application_owner();
                let player_index = self.state.players.iter().position(|p| p.owner == owner)
                    .ok_or("Player not in room".to_string())?;

                // Deduct bet from player balance (simulate bankroll transfer)
                if self.state.players[player_index].balance < bet_amount {
                    return Err("Insufficient balance for bet".to_string());
                }
                self.state.players[player_index].balance -= bet_amount;

                // Start the round with fixed question (replace with random later)
                let question = Question {
                    text: "What is the capital of France?".to_string(),
                    options: vec![
                        "Berlin".to_string(),
                        "Paris".to_string(),
                        "Madrid".to_string(),
                        "London".to_string(),
                    ],
                    correct_index: 1,
                };
                self.state.current_question = Some(question);
                self.state.answers.clear();
                self.state.round_active = true;
                Ok(())
            }

            Operation::SubmitAnswer { answer_index } => {
                if !self.state.round_active {
                    return Err("No active round".to_string());
                }
                let owner = self.runtime.application_owner();
                if !self.state.players.iter().any(|p| p.owner == owner) {
                    return Err("Not in room".to_string());
                }
                if let Some(question) = &self.state.current_question {
                    if answer_index >= question.options.len() {
                        return Err("Invalid answer index".to_string());
                    }
                    let chain_id = self.runtime.chain_id();
                    self.state.answers.push((chain_id, answer_index));
                }
                Ok(())
            }

            Operation::EndRound => {
                if !self.state.round_active {
                    return Err("No active round to end".to_string());
                }

                if let Some(question) = &self.state.current_question {
                    let correct_index = question.correct_index;

                    // Resolve answers and pay winners
                    for (chain_id, answer_index) in &self.state.answers {
                        // Find player (simplified matching by chain_id)
                        if let Some(player) = self.state.players.iter_mut().find(|p| {
                            // Replace with proper chain_id matching in production
                            true // MVP placeholder
                        }) {
                            if *answer_index == correct_index {
                                // Payout: 2x bet (fixed 200 tokens for MVP)
                                player.balance += 200;
                                player.score += 1;
                            }
                        }
                    }
                }

                // Clean up round
                self.state.round_active = false;
                self.state.current_question = None;
                self.state.answers.clear();
                Ok(())
            }
        }
    }

    async fn execute_message(&mut self, _message: Message) -> Result<(), String> {
        Ok(())
    }
}
