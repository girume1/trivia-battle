#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{Service, ServiceRuntime, views::View};
use state::MasterState;

pub struct MasterService {
    state: Arc<MasterState>,
}

linera_sdk::service!(MasterService);

impl linera_sdk::abi::WithServiceAbi for MasterService {
    type Abi = crate::MasterAbi;
}

#[async_trait::async_trait]
impl Service for MasterService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = MasterState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { state: Arc::new(state) }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot { state: self.state.clone() },
            async_graphql::EmptyMutation,
            EmptySubscription,
        ).finish();
        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<MasterState>,
}

#[Object]
impl QueryRoot {
    async fn treasury_balance(&self) -> Amount {
        *self.state.treasury.get()
    }
}