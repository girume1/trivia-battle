use linera_sdk::views::{RegisterView, RootView, ViewStorageContext};
use abi::trivia::Question;
use linera_sdk::linera_base_types::{Amount, AccountOwner};
use serde::{Deserialize, Serialize};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct MasterState {
    pub question_bank: RegisterView<Vec<Question>>,
    pub next_question_id: RegisterView<u64>,
    pub treasury: RegisterView<Amount>,
    pub admin: RegisterView<Option<AccountOwner>>,
}