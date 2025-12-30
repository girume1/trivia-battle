pub mod state;

use linera_sdk::abi::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};
use async_graphql::{Request, Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct MasterAbi;

impl ContractAbi for MasterAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for MasterAbi {
    type Query = Request;
    type QueryResponse = Response;
}