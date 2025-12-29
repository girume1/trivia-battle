#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use state::BankrollState;

pub struct BankrollService {
    state: Arc<BankrollState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(BankrollService);

impl WithServiceAbi for BankrollService {
    type Abi = crate::BankrollAbi;
}

impl Service for BankrollService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = BankrollState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");

        BankrollService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            crate::BankrollOperation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();

        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<BankrollState>,
}

#[Object]
impl QueryRoot {
    async fn balance(&self, owner: AccountOwner) -> Amount {
        self.state.accounts.get(&owner).await
            .expect("Failed to read")
            .unwrap_or(Amount::ZERO)
    }

    async fn daily_bonus_amount(&self) -> Amount {
        self.runtime.application_parameters().bonus
    }

    async fn total_pot(&self) -> Amount {
        // Sum all pots or track separately
        Amount::ZERO // Placeholder — improve if needed
    }

    async fn public_chain_balances(&self) -> Vec<(ChainId, Amount)> {
        let mut list = Vec::new();
        let keys = self.state.public_balances.indices().await.unwrap_or_default();
        for key in keys {
            if let Some(amount) = self.state.public_balances.get(&key).await.unwrap() {
                list.push((key, amount));
            }
        }
        list
    }
}