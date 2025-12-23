use linera_sdk::{
    base::{AccountOwner, ChainId},
    Contract, ContractRuntime, WithContractRuntime,
};
use serde::{Deserialize, Serialize};
use crate::{BankrollState, Account};

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Mint { amount: u64 },
    Transfer { to: AccountOwner, amount: u64 },
    CreditDailyBonus,
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    // Future: cross-chain transfers (e.g., between chains)
}

pub struct BankrollContract {
    runtime: ContractRuntime<Self>,
    state: BankrollState,
}

impl WithContractRuntime for BankrollContract {
    fn runtime(&self) -> &ContractRuntime<Self> { &self.runtime }
    fn runtime_mut(&mut self) -> &mut ContractRuntime<Self> { &mut self.runtime }
}

impl Contract for BankrollContract {
    type Message = Message;
    type Parameters = ();
    type ApplicationCall = Operation;

    fn new(runtime: ContractRuntime<Self>) -> Self {
        Self {
            runtime,
            state: BankrollState::default(),
        }
    }

    async fn initialize(&mut self, _parameters: (), call: Operation) -> Result<(), String> {
        match call {
            Operation::Mint { amount } => {
                if amount == 0 {
                    return Err("Mint amount must be greater than zero".to_string());
                }
                let owner = self.runtime.application_owner();
                // Prevent double-minting for the same owner
                if self.state.accounts.iter().any(|a| a.owner == owner) {
                    return Err("Account already minted".to_string());
                }
                self.state.total_supply += amount;
                self.state.accounts.push(Account { owner, balance: amount });
                Ok(())
            }
            _ => Err("Initialize only supports Mint operation".to_string()),
        }
    }

    async fn execute_operation(&mut self, operation: Operation) -> Result<(), String> {
        let owner = self.runtime.application_owner();
        let account_index = self.state.accounts.iter().position(|a| a.owner == owner)
            .ok_or("Account not found. Mint tokens first.".to_string())?;

        match operation {
            Operation::Transfer { to, amount } => {
                if amount == 0 {
                    return Err("Transfer amount must be greater than zero".to_string());
                }
                if amount > self.state.accounts[account_index].balance {
                    return Err("Insufficient balance".to_string());
                }
                self.state.accounts[account_index].balance -= amount;

                // Credit recipient (create account if it doesn't exist)
                if let Some(recipient) = self.state.accounts.iter_mut().find(|a| a.owner == to) {
                    recipient.balance += amount;
                } else {
                    self.state.accounts.push(Account { owner: to, balance: amount });
                }
                Ok(())
            }

            Operation::CreditDailyBonus => {
                // MVP: simple bonus (in production, add timestamp check for daily limit)
                self.state.accounts[account_index].balance += 100;
                Ok(())
            }

            _ => Err("Unsupported operation".to_string()),
        }
    }

    async fn execute_message(&mut self, _message: Message) -> Result<(), String> {
        // Reserved for cross-chain communication
        Ok(())
    }
}
