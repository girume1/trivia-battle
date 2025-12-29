use async_graphql::{Request, Response};
use bankroll::BankrollAbi;
use linera_sdk::linera_base_types::{
    AccountOwner, Amount, ApplicationId, ChainId, Timestamp,
};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

// Shared ABI types
use abi::management::{PublicChainInfo, RoomInfo};
use abi::trivia::{Question, QuestionInput, TriviaGame}; // Added QuestionInput

#[derive(Debug, Deserialize, Serialize)]
pub struct TriviaAbi;

impl ContractAbi for TriviaAbi {
    type Operation = TriviaOperation;
    type Response = ();
}

impl ServiceAbi for TriviaAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum TriviaOperation {
    // User actions
    InitialSetup {},
    FindPlayChain {}, // Matchmaking
    OpenRoom {
        name: String,
        max_players: u8,
        bet_amount: Amount,
        password: Option<String>,
        display_name: String,
    },
    RequestJoinRoom {
        room_chain: ChainId,
        password: Option<String>,
        display_name: String,
    },
    StartGame {},
    Answer {
        question_index: u8,
        choice: u8,
    },
    LeaveRoom {},

    // Admin (Master only)
    AddPublicChain {
        chain_id: ChainId,
        initial_funding: Amount,
    },
    AddRoomManagerChain {
        chain_id: ChainId,
    },
    AddQuestions {
        questions: Vec<QuestionInput>, // Changed to QuestionInput
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TriviaMessage {
    GameUpdate { game: TriviaGame },
    JoinResult {
        success: bool,
        message: String,
        room_chain: Option<ChainId>,
    },
    PublicChainsData { chains: Vec<PublicChainInfo> },
    RoomsData { rooms: Vec<RoomInfo> },

    PlayerJoined { player: AccountOwner, name: String },
    PlayerLeft { player: AccountOwner },
    PlayerAnswered {
        player: AccountOwner,
        question_index: u8,
        choice: u8,
        answered_at: Timestamp,
    },
    GameStarted { question_ids: Vec<u64> },
    NextQuestion { index: u8, question_id: u64 },
    GameEnded { winner: AccountOwner, payout: Amount },

    RequestQuestions { count: u8 },
    ReceiveQuestions { question_ids: Vec<u64>, questions: Vec<Question> },

    FindPlayChainRequest { player: AccountOwner },

    SendProtocolFee { amount: Amount },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TriviaParameters {
    pub master_chain: ChainId,
    pub bankroll_app: ApplicationId<BankrollAbi>,
}