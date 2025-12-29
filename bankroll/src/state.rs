use linera_sdk::linera_base_types::{AccountOwner, Amount, ChainId};
use linera_sdk::views::{MapView, RegisterView, RootView, ViewStorageContext};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct BankrollState {
    // User balances
    pub accounts: MapView<AccountOwner, Amount>,

    // Daily bonus
    pub daily_bonus: RegisterView<DailyBonus>,

    // Debts and pots
    pub debt_log: MapView<u64, DebtRecord>,
    pub token_pot_log: MapView<u64, TokenPotRecord>,

    // Public chain tracking
    pub public_balances: MapView<ChainId, Amount>,
}