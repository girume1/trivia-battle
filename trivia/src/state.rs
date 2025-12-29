use crate::game::{TriviaBattle, PlayerInBattle, BattleStatus};
use abi::bet_chip_profile::Profile;
use abi::leaderboard::SimpleLeaderboardEntry;
use abi::management::{PublicChainInfo, RoomInfo};
use abi::trivia::UserStatus;
use linera_sdk::linera_base_types::{Amount, ChainId};
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct TriviaState {
    // Global
    pub public_chains: MapView<ChainId, PublicChainInfo>,
    pub rooms: MapView<u64, RoomInfo>,
    pub leaderboard: RegisterView<Vec<SimpleLeaderboardEntry>>,

    // User Chain
    pub profile: RegisterView<Profile>,
    pub user_status: RegisterView<UserStatus>,
    pub current_room: RegisterView<Option<ChainId>>,

    // Play Chain (this is where the battle happens)
    pub battle: RegisterView<TriviaBattle>,

    // Public Chain (lobby)
    pub public_info: RegisterView<PublicChainInfo>,

    pub leaderboard: RegisterView<Vec<LeaderboardEntry>>,
}