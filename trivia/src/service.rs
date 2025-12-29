#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;
mod game;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use state::TriviaState;
use game::{TriviaBattle, PlayerInBattle};

// === ADD THESE LINES HERE ===
use abi::management::{PublicChainInfo, RoomInfo};
use abi::trivia::UserStatus;
use abi::leaderboard::SimpleLeaderboardEntry;
use linera_sdk::linera_base_types::ChainId;
// ============================

pub struct TriviaService {
    state: Arc<TriviaState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(TriviaService);

impl WithServiceAbi for TriviaService {
    type Abi = crate::TriviaAbi;
}

impl Service for TriviaService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TriviaState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");

        TriviaService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            crate::TriviaOperation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();

        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<TriviaState>,
}

#[Object]
impl QueryRoot {
    // Get current battle (for Play Chain)
    async fn current_battle(&self) -> TriviaBattle {
        self.state.battle.get().clone()
    }

    // Get player's current room (for User Chain)
    async fn my_current_room(&self) -> Option<ChainId> {
        self.state.current_room.get().clone()
    }

    // Get profile & stats
    async fn my_profile(&self) -> abi::bet_chip_profile::Profile {
        self.state.profile.get().clone()
    }

    async fn my_user_status(&self) -> abi::trivia::UserStatus {
        self.state.user_status.get().clone()
    }

    // Global data
    async fn all_rooms(&self) -> Vec<RoomInfo> {
        let mut rooms = Vec::new();
        let keys = self.state.rooms.indices().await.unwrap_or_default();
        for key in keys {
            if let Some(room) = self.state.rooms.get(&key).await.unwrap_or(None) {
                rooms.push(room);
            }
        }
        rooms
    }

    async fn leaderboard(&self) -> Vec<abi::leaderboard::SimpleLeaderboardEntry> {
        self.state.leaderboard.get().clone()
    }

    async fn public_chains(&self) -> Vec<PublicChainInfo> {
        let mut chains = Vec::new();
        let keys = self.state.public_chains.indices().await.unwrap_or_default();
        for key in keys {
            if let Some(info) = self.state.public_chains.get(&key).await.unwrap_or(None) {
                chains.push(info);
            }
        }
        chains
    }

    // For Public Chain lobby view
    async fn my_public_info(&self) -> PublicChainInfo {
        self.state.public_info.get().clone()
    }
}