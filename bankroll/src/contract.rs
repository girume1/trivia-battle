#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_trait::async_trait;
use linera_sdk::{
    views::View,
    Contract, ContractRuntime,
    linera_base_types::{AccountOwner, Amount, ChainId, Timestamp},
};
use state::BankrollState;
use crate::{BankrollOperation, BankrollMessage, BankrollResponse, BankrollParameters};
use bcs;

pub struct BankrollContract {
    runtime: ContractRuntime<Self>,
    state: BankrollState,
}

linera_sdk::contract!(BankrollContract);

#[async_trait]
impl Contract for BankrollContract {
    type Message = BankrollMessage;
    type Parameters = BankrollParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = BankrollState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { runtime, state }
    }

    async fn store(self) {
        self.state.save().await.expect("Failed to save state");
    }

    async fn instantiate(&mut self, _argument: ()) {
        // Parameters are validated automatically
    }

    async fn execute_operation(&mut self, operation: BankrollOperation) -> BankrollResponse {
        match operation {
            BankrollOperation::Balance { owner } => {
                let balance = self.state.accounts.get(&owner).await
                    .expect("Failed to read balance")
                    .unwrap_or(Amount::ZERO);

                // Claim daily bonus if eligible
                let mut daily = self.state.daily_bonus.get_mut();
                let current_time = self.runtime.system_time();
                let bonus_amount = daily.claim_bonus(current_time, self.runtime.application_parameters().bonus);

                if !bonus_amount.is_zero() {
                    let new_balance = balance.saturating_add(bonus_amount);
                    self.state.accounts.insert(owner, new_balance).unwrap();
                    return BankrollResponse::Balance(new_balance);
                }

                BankrollResponse::Balance(balance)
            }

            BankrollOperation::UpdateBalance { owner, amount } => {
                // Only allow trusted apps (like trivia) or master to update
                // In production: add auth check
                self.state.accounts.insert(owner, amount).unwrap_or_else(|_| panic!("Insert failed"));
                BankrollResponse::Ok
            }

            BankrollOperation::NotifyDebt { amount, target_chain } => {
                let debt_id = self.state.next_debt_id.get();
                self.state.next_debt_id.set(debt_id + 1);

                let debt_record = DebtRecord {
                    id: debt_id,
                    amount,
                    target_chain,
                    created_at: self.runtime.system_time(),
                    status: DebtStatus::Pending,
                };

                self.state.debt_log.insert(debt_id, debt_record).unwrap();

                // Notify target chain
                self.send_message(target_chain, BankrollMessage::DebtNotif {
                    debt_id,
                    amount,
                    created_at: self.runtime.system_time(),
                });

                BankrollResponse::Ok
            }

            BankrollOperation::TransferPot { amount, target_chain } => {
                // Add to pot and notify
                let pot_id = self.state.next_pot_id.get();
                self.state.next_pot_id.set(pot_id + 1);

                let pot_record = TokenPotRecord {
                    id: pot_id,
                    amount,
                    target_chain,
                    created_at: self.runtime.system_time(),
                };
                self.state.token_pot_log.insert(pot_id, pot_record).unwrap();

                self.send_message(target_chain, BankrollMessage::TokenPot { amount });

                BankrollResponse::Ok
            }
        }
    }

    async fn execute_message(&mut self, message: BankrollMessage) {
        match message {
            BankrollMessage::DebtPaid { debt_id, amount, paid_at } => {
                if let Some(mut debt) = self.state.debt_log.get(&debt_id).await.unwrap() {
                    debt.status = DebtStatus::Paid;
                    debt.paid_at = Some(paid_at);
                    self.state.debt_log.insert(debt_id, debt).unwrap();
                }
            }
            BankrollMessage::TokenUpdate { amount } => {
                let origin = self.runtime.message_sender().expect("Sender required");
                self.state.public_balances.insert(origin, amount).unwrap();
            }
            _ => {}
        }
    }
}

impl BankrollContract {
    fn send_message(&mut self, destination: ChainId, message: BankrollMessage) {
        self.runtime.prepare_message(message).with_tracking().send_to(destination);
    }
}