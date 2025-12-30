#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_trait::async_trait;
use linera_sdk::{
    Contract, ContractRuntime,
    linera_base_types::{AccountOwner, Amount},
    abi::{WithContractAbi},
    views::View,
};
use state::MasterState;
use trivia::TriviaMessage;
use abi::trivia::Question;

pub struct MasterContract {
    runtime: ContractRuntime<Self>,
    state: MasterState,
}

linera_sdk::contract!(MasterContract);

impl WithContractAbi for MasterContract {
    type Abi = crate::MasterAbi;
}

#[async_trait]
impl Contract for MasterContract {
    type Message = TriviaMessage;
    type Parameters = ();
    type InstantiationArgument = (Vec<Question>, AccountOwner);
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = MasterState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { runtime, state }
    }

    async fn store(self) {
        self.state.save().await.expect("Failed to save state");
    }

    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        let (questions, admin_owner) = argument;
        self.state.question_bank.set(questions);
        self.state.admin.set(Some(admin_owner));
        self.state.treasury.set(Amount::ZERO);
    }

    async fn execute_operation(&mut self, _op: Self::Operation) -> Self::Response {}

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            TriviaMessage::RequestQuestions { count } => {
                let bank = self.state.question_bank.get();
                let mut selected = Vec::new();
                let mut ids = Vec::new();
                for i in 0..count.min(bank.len() as u8) {
                    let q = &bank[i as usize];
                    ids.push(q.id);
                    selected.push(q.clone());
                }
                let origin = self.runtime.message_sender().expect("No sender");
                self.runtime.prepare_message(TriviaMessage::ReceiveQuestions {
                    question_ids: ids,
                    questions: selected,
                })
                .send_to(origin);
            }
            TriviaMessage::SendProtocolFee { amount } => {
                let mut treasury = self.state.treasury.get_mut();
                treasury.saturating_add_assign(amount);
            }
            _ => {}
        }
    }
}