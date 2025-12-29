use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{AccountOwner, Amount, ChainId, Timestamp};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BankrollAbi;

impl ContractAbi for BankrollAbi {
    type Operation = BankrollOperation;
    type Response = BankrollResponse;
}

impl ServiceAbi for BankrollAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum BankrollOperation {
    Balance { owner: AccountOwner },
    UpdateBalance { owner: AccountOwner, amount: Amount },
    NotifyDebt { amount: Amount, target_chain: ChainId },
    TransferPot { amount: Amount, target_chain: ChainId },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BankrollMessage {
    TokenPot { amount: Amount },
    DebtNotif { debt_id: u64, amount: Amount, created_at: Timestamp },
    DebtPaid { debt_id: u64, amount: Amount, paid_at: Timestamp },
    TokenUpdate { amount: Amount },
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum BankrollResponse {
    #[default]
    Ok,
    Balance(Amount),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BankrollParameters {
    pub master_chain: ChainId,
    pub bonus: Amount,
}