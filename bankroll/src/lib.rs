use linera_sdk::base::{AccountOwner, ChainId};
use serde::{Deserialize, Serialize};

/// Represents a single user's account in the bankroll system
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Account {
    /// The owner of this account (user chain/application owner)
    pub owner: AccountOwner,
    /// Current token balance
    pub balance: u64,
}

/// Global state of the bankroll application
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BankrollState {
    /// List of all accounts (user balances)
    pub accounts: Vec<Account>,
    /// Total amount of tokens ever minted (for reference/auditing)
    pub total_supply: u64,
}

/// Operations supported by the bankroll contract
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BankrollOperation {
    Mint { amount: u64 },
    Transfer { to: AccountOwner, amount: u64 },
    CreditDailyBonus,
}
