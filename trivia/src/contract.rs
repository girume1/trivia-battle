#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;
mod game;

use async_trait::async_trait;
use linera_sdk::{
    views::View,
    Contract, ContractRuntime,
    linera_base_types::{AccountOwner, Amount, Timestamp, ChainId},
};
use state::TriviaState;
use game::{TriviaBattle, PlayerInBattle, BattleStatus};
use crate::{TriviaOperation, TriviaMessage, TriviaParameters};
use bankroll::BankrollOperation;
use abi::leaderboard::LeaderboardEntry;

pub struct TriviaContract {
    runtime: ContractRuntime<Self>,
    state: TriviaState,
}

linera_sdk::contract!(TriviaContract);

#[async_trait]
impl Contract for TriviaContract {
    type Message = TriviaMessage;
    type Parameters = TriviaParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TriviaState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { runtime, state }
    }

    async fn store(self) {
        self.state.save().await.expect("Failed to save state");
    }

    async fn instantiate(&mut self, _arg: ()) {}

    async fn execute_operation(&mut self, op: TriviaOperation) {
        let signer = match self.runtime.authenticated_signer() {
            Some(s) => s,
            None => return,
        };

        match op {
            TriviaOperation::OpenRoom {
                name,
                max_players,
                bet_amount,
                password,
                display_name,
            } => {
                let mut battle = TriviaBattle {
                    room_name: name,
                    owner: signer,
                    max_players,
                    bet_amount,
                    password,
                    players: vec![],
                    question_ids: vec![],
                    full_questions: vec![],
                    current_question_index: 0,
                    current_question_start_time: None,
                    question_timeout_seconds: 30,
                    status: BattleStatus::Waiting,
                    pot: Amount::ZERO,
                    start_time: None,
                };

                // Owner joins with real display name
                battle.players.push(PlayerInBattle {
                    owner: signer,
                    name: display_name,
                    score: 0,
                    has_answered_current: false,
                    last_answer_time: None,
                });

                self.state.battle.set(battle);
            }

            TriviaOperation::RequestJoinRoom {
                room_chain: _,
                password,
                display_name,
            } => {
                let mut battle = self.state.battle.get_mut();

                if battle.status != BattleStatus::Waiting {
                    return;
                }

                // Password check
                if let Some(ref pwd) = battle.password {
                    if Some(pwd.as_str()) != password.as_deref() {
                        return;
                    }
                }

                // Room full?
                if battle.players.len() >= battle.max_players as usize {
                    return;
                }

                // Player joins with real display name
                battle.players.push(PlayerInBattle {
                    owner: signer,
                    name: display_name,
                    score: 0,
                    has_answered_current: false,
                    last_answer_time: None,
                });

                self.broadcast(TriviaMessage::PlayerJoined {
                    player: signer,
                    name: display_name.clone(),
                });
            }

            TriviaOperation::StartGame {} => {
                let mut battle = self.state.battle.get_mut();

                if battle.owner != signer || battle.status != BattleStatus::Waiting {
                    return;
                }

                if battle.players.len() < 2 {
                    return;
                }

                let bet = battle.bet_amount;

                // === REAL BET COLLECTION ===
                if !bet.is_zero() {
                    let bankroll_id = self.runtime.application_parameters().bankroll_app;

                    for player in &battle.players {
                        let debt_op = BankrollOperation::NotifyDebt {
                            amount: bet,
                            target_chain: self.runtime.chain_id(),
                        };
                        self.runtime.call_application(true, bankroll_id, &debt_op);
                    }

                    battle.pot = bet.checked_mul(battle.players.len() as u128).unwrap_or(Amount::MAX);
                }

                battle.status = BattleStatus::InProgress;
                battle.start_time = Some(self.runtime.system_time());

                let master = self.runtime.application_parameters().master_chain;
                self.send_message(master, TriviaMessage::RequestQuestions { count: 10 });
            }

            TriviaOperation::Answer { question_index, choice } => {
                let mut battle = self.state.battle.get_mut();

                if battle.status != BattleStatus::InProgress
                    || battle.current_question_index != question_index
                {
                    return;
                }

                let player_idx = match battle.players.iter().position(|p| p.owner == signer) {
                    Some(i) => i,
                    None => return,
                };

                let player = &mut battle.players[player_idx];
                if player.has_answered_current {
                    return;
                }

                player.has_answered_current = true;
                player.last_answer_time = Some(self.runtime.system_time());

                let question = &battle.full_questions[question_index as usize];
                let is_correct = choice == question.correct_idx;
                let points = if is_correct { 100 } else { 0 };
                let speed_bonus = 20;
                player.score += points + speed_bonus;

                self.broadcast(TriviaMessage::PlayerAnswered {
                    player: signer,
                    question_index,
                    choice,
                    answered_at: self.runtime.system_time(),
                });

                self.check_question_timeout_and_advance();
            }

            _ => {}
        }
    }

    async fn execute_message(&mut self, message: TriviaMessage) {
        let mut battle = self.state.battle.get_mut();

        match message {
            TriviaMessage::ReceiveQuestions { question_ids, questions } => {
                if battle.status != BattleStatus::InProgress {
                    return;
                }

                battle.question_ids = question_ids;
                battle.full_questions = questions;
                battle.current_question_index = 0;
                battle.current_question_start_time = Some(self.runtime.system_time());

                for p in &mut battle.players {
                    p.has_answered_current = false;
                    p.last_answer_time = None;
                }

                self.broadcast(TriviaMessage::GameStarted {
                    question_ids: battle.question_ids.clone(),
                });

                self.broadcast(TriviaMessage::NextQuestion {
                    index: 0,
                    question_id: battle.question_ids[0],
                });
            }
            _ => {}
        }
    }
}

impl TriviaContract {
    fn broadcast(&mut self, msg: TriviaMessage) {
        log::info!("Broadcast: {:?}", msg);
    }

    fn send_message(&mut self, destination: ChainId, msg: TriviaMessage) {
        self.runtime.prepare_message(msg).send_to(destination);
    }

    fn check_question_timeout_and_advance(&mut self) {
        let mut battle = self.state.battle.get_mut();
        let now = self.runtime.system_time();

        let timeout = if let Some(start) = battle.current_question_start_time {
            (now.microseconds() - start.microseconds()) >= (battle.question_timeout_seconds as i128 * 1_000_000)
        } else {
            false
        };

        let all_answered = battle.players.iter().all(|p| p.has_answered_current);

        if all_answered || timeout {
            self.advance_to_next_question();
        }
    }

    fn advance_to_next_question(&mut self) {
        let mut battle = self.state.battle.get_mut();
        battle.current_question_index += 1;
        battle.current_question_start_time = Some(self.runtime.system_time());

        if battle.current_question_index as usize >= battle.question_ids.len() {
            self.end_game();
            return;
        }

        for p in &mut battle.players {
            p.has_answered_current = false;
            p.last_answer_time = None;
        }

        self.broadcast(TriviaMessage::NextQuestion {
            index: battle.current_question_index,
            question_id: battle.question_ids[battle.current_question_index as usize],
        });
    }

    fn end_game(&mut self) {
        let mut battle = self.state.battle.get_mut();
        battle.status = BattleStatus::Finished;

        // Find winner
        let winner_info = battle.players.iter()
            .max_by_key(|p| p.score)
            .expect("No players");

        let winner = winner_info.owner;
        let winner_score = winner_info.score;
        let winner_name = winner_info.name.clone();

        // Calculate payouts
        let fee = battle.pot / 20; // 5%
        let base_payout = battle.pot.saturating_sub(fee);

        // === TIER MULTIPLIER (SIMPLIFIED) ===
        let multiplier = self.get_tier_multiplier(winner_score);
        let final_payout = base_payout * multiplier / 100;

        // === UPDATE GLOBAL LEADERBOARD ===
        let mut entries = self.state.leaderboard.get_mut().clone();
        if let Some(entry) = entries.iter_mut().find(|e| e.player == winner) {
            entry.wins += 1;
            entry.total_score += winner_score as u64;
            entry.lifetime_winnings.saturating_add_assign(final_payout);
        } else {
            entries.push(LeaderboardEntry {
                player: winner,
                name: winner_name,
                wins: 1,
                total_score: winner_score as u64,
                lifetime_winnings: final_payout,
            });
        }

        // Sort and keep top 100
        entries.sort_by_key(|e| std::cmp::Reverse(e.wins));
        if entries.len() > 100 {
            entries.truncate(100);
        }
        self.state.leaderboard.set(entries);

        self.broadcast(TriviaMessage::GameEnded { winner, payout: final_payout });

        // Send fee to Master
        if !fee.is_zero() {
            let master = self.runtime.application_parameters().master_chain;
            self.send_message(master, TriviaMessage::SendProtocolFee { amount: fee });
        }

        // Payout winner
        if !final_payout.is_zero() {
            let bankroll_id = self.runtime.application_parameters().bankroll_app;
            let payout_op = BankrollOperation::UpdateBalance {
                owner: winner,
                amount: final_payout,
            };
            self.runtime.call_application(true, bankroll_id, &payout_op);
        }

        battle.pot = Amount::ZERO;
    }

    fn get_tier_multiplier(&self, score: u64) -> u32 {
        // Simple tier based on total score across games
        if score >= 1000 { 200 }      // Diamond 2x
        else if score >= 500 { 150 }  // Gold 1.5x
        else if score >= 200 { 125 }  // Silver 1.25x
        else { 100 }                  // Bronze 1x
    }
}