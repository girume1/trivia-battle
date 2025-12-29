use async_trait::async_trait;
use linera_sdk::{
    Contract, ContractRuntime,
    linera_base_types::{AccountOwner, Amount},
    abi::WithContractAbi,
};
use super::state::MasterState;
use trivia::TriviaMessage;
use abi::trivia::{Question, Operation, MasterAbi};

pub struct MasterContract {
    runtime: ContractRuntime<Self>,
    state: MasterState,
}

impl WithContractAbi for MasterContract {
    type Abi = MasterAbi;
}

#[async_trait] // Fixes E0195
impl Contract for MasterContract {
    type Message = TriviaMessage;
    type Parameters = ();
    type InstantiationArgument = (Vec<Question>, AccountOwner);
    type EventValue = ();

    async fn store(mut self) { 
        self.state.save().await.expect("Failed to save state");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }

    async fn instantiate(&mut self, (questions, admin_owner): Self::InstantiationArgument) {
        let max_id = questions.iter().map(|q| q.id).max().unwrap_or(0);
        self.state.question_bank.set(questions);
        self.state.next_question_id.set(max_id + 1);
        self.state.treasury.set(Amount::ZERO);
        self.state.admin.set(Some(admin_owner));
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::AddQuestion { mut question } => {
                self.ensure_admin();
                let mut bank = self.state.question_bank.get().clone();
                let next_id = *self.state.next_question_id.get();
                question.id = next_id;
                bank.push(question);
                self.state.question_bank.set(bank);
                self.state.next_question_id.set(next_id + 1);
            }
            Operation::Withdraw { amount } => {
                self.ensure_admin();
                let mut treasury = *self.state.treasury.get();
                if treasury >= amount {
                    let signer = self.runtime.authenticated_signer().expect("No signer");
                    treasury = treasury.saturating_sub(amount);
                    self.state.treasury.set(treasury);
                    
                    let dest_account = Account { 
                        chain_id: self.runtime.chain_id(), 
                        owner: signer 
                    };
                    self.runtime.transfer(signer, dest_account, amount);
                }
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            TriviaMessage::RequestQuestions { count } => {
                let bank = self.state.question_bank.get();
                if bank.is_empty() { return; }

                let mut selected_questions = Vec::new();
                let mut selected_ids = Vec::new();
                let seed = self.runtime.block_height().0 as usize;
                let total = bank.len();

                for i in 0..count.min(total as u8) {
                    let idx = (seed + i as usize) % total;
                    let q = &bank[idx];
                    selected_ids.push(q.id);
                    selected_questions.push(q.clone());
                }

                // Fix: Use message_sender() to get the origin chain
                let origin_chain = self.runtime.message_sender().expect("No sender").chain_id;
                self.runtime.prepare_message(TriviaMessage::ReceiveQuestions {
                    question_ids: selected_ids,
                    questions: selected_questions,
                })
                .send_to(origin_chain);
            }
            TriviaMessage::SendProtocolFee { amount } => {
                let mut treasury = *self.state.treasury.get();
                self.state.treasury.set(treasury.saturating_add(amount));
            }
            _ => {}
        }
    }
}

impl MasterContract {
    fn ensure_admin(&mut self) {
        let signer = self.runtime.authenticated_signer().expect("No signer");
        let admin = self.state.admin.get().expect("No admin set");
        assert_eq!(signer, admin, "Only admin allowed");
    }
}