#![cfg_attr(target_arch = "wasm32", no_main)]

mod contract;
mod state;

use contract::MasterContract;

linera_sdk::contract!(MasterContract);