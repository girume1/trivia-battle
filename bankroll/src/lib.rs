use linera_sdk::base::{AccountOwner, ChainId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub owner: AccountOwner,
    pub balance: u64,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BankrollState {
    pub accounts: Vec<Account>,
    pub total_supply: u64,
}
