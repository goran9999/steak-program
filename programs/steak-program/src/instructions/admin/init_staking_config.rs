use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{state::StakingConfig, utilities::STEAK_SEED};

#[derive(Accounts)]
#[instruction(collection_symbol:String)]
pub struct InitStaking<'info> {
    #[account(init,seeds=[collection_symbol.as_bytes(),
    collection.key().as_ref(),update_authority.key().as_ref(),STEAK_SEED],bump,
    payer=update_authority,space=8+size_of::<StakingConfig>())]
    pub staking_config: Account<'info, StakingConfig>,
    #[account(mut)]
    pub update_authority: Signer<'info>,
    #[account()]
    pub collection: Account<'info, Mint>,
    #[account()]
    ///CHECK:check in ix
    pub collection_metadata: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_staking_config(
    ctx: Context<InitStaking>,
    collection_symbol: String,
    staking_period: u8,
    staking_reward: u64,
    staking_increase: u8,
    increase_amount: Option<u64>,
) {
}
