use anchor_lang::prelude::*;

#[event]
pub struct DepositRewardEvent {
    pub deposit_amount: u64,
    pub deposit_timestamp: i64,
    pub deposit_mint: Pubkey,
}
