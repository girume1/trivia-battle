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
    StartRound,
    SubmitAnswer { answer_index: usize },
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
                    balance: 1000, // Starting balance for future betting
                    score: 0,
                });
                Ok(())
            }

            Operation::StartRound => {
                if self.state.round_active {
                    return Err("Round already active".to_string());
                }
                // MVP: Fixed question (later: random or oracle)
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
        }
    }

    async fn execute_message(&mut self, _message: Message) -> Result<(), String> {
        Ok(())
    }
}
