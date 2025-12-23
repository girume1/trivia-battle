use async_graphql::{EmptySubscription, Object, Schema, SimpleObject};
use linera_sdk::service::{Service, ServiceRuntime};
use crate::BankrollState;

/// View returned by the balance query
#[derive(SimpleObject)]
struct BalanceView {
    balance: u64,
}

/// Optional: Add a total supply query for transparency
#[derive(SimpleObject)]
struct SupplyView {
    total_supply: u64,
}

pub struct BankrollService {
    runtime: ServiceRuntime<super::contract::BankrollContract>,
}

#[Object]
impl BankrollService {
    /// Get the balance of the current user
    async fn balance(&self) -> BalanceView {
        let state = self.runtime.application_state();
        let owner = self.runtime.application_owner();
        let balance = state.accounts
            .iter()
            .find(|a| a.owner == owner)
            .map(|a| a.balance)
            .unwrap_or(0);
        BalanceView { balance }
    }

    /// Get the total supply of tokens (useful for auditing)
    async fn total_supply(&self) -> SupplyView {
        let state = self.runtime.application_state();
        SupplyView {
            total_supply: state.total_supply,
        }
    }
}

impl Service for BankrollService {
    type Query = Schema<Self, EmptySubscription, EmptySubscription>;

    fn new(runtime: ServiceRuntime<super::contract::BankrollContract>) -> Self {
        Self { runtime }
    }

    async fn query(&self, request: async_graphql::Request) -> async_graphql::Response {
        let schema = Schema::build(Self, EmptySubscription, EmptySubscription).finish();
        schema.execute(request).await
    }
}
