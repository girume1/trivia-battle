use async_graphql::{EmptySubscription, Object, Schema, SimpleObject};
use linera_sdk::service::{Service, ServiceRuntime};
use crate::GameState;

#[derive(SimpleObject)]
struct StateView {
    room_id: String,
    players_count: usize,
    round_active: bool,
    current_question: Option<String>,
    answers_count: usize,
}

#[derive(SimpleObject)]
struct PlayerInfo {
    balance: u64,
    score: u32,
}

pub struct TriviaService {
    runtime: ServiceRuntime<super::contract::TriviaContract>,
}

#[Object]
impl TriviaService {
    async fn state(&self) -> StateView {
        let state = self.runtime.application_state();
        StateView {
            room_id: state.room_id.clone(),
            players_count: state.players.len(),
            round_active: state.round_active,
            current_question: state.current_question.as_ref().map(|q| q.text.clone()),
            answers_count: state.answers.len(),
        }
    }

    async fn player_balance(&self) -> u64 {
        let state = self.runtime.application_state();
        let owner = self.runtime.application_owner();
        state.players
            .iter()
            .find(|p| p.owner == owner)
            .map(|p| p.balance)
            .unwrap_or(0)
    }

    async fn player_score(&self) -> u32 {
        let state = self.runtime.application_state();
        let owner = self.runtime.application_owner();
        state.players
            .iter()
            .find(|p| p.owner == owner)
            .map(|p| p.score)
            .unwrap_or(0)
    }

    async fn total_pot(&self) -> u64 {
        let state = self.runtime.application_state();
        // Simple MVP: sum of all player bets (assuming bet is stored or fixed)
        // In full version, track a pot field in GameState
        state.players.iter().map(|p| p.balance).sum::<u64>() // Placeholder: replace with real pot
    }
}

impl Service for TriviaService {
    type Query = Schema<Self, EmptySubscription, EmptySubscription>;

    fn new(runtime: ServiceRuntime<super::contract::TriviaContract>) -> Self {
        Self { runtime }
    }

    async fn query(&self, request: async_graphql::Request) -> async_graphql::Response {
        let schema = Schema::build(Self, EmptySubscription, EmptySubscription).finish();
        schema.execute(request).await
    }
}
