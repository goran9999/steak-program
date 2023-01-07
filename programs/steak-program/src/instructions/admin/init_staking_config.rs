use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program::borsh::try_from_slice_unchecked};
use anchor_spl::token::Mint;
use num_enum::TryFromPrimitive;

use crate::{
    state::{CustomStakingIncrease, StakingConfig, StakingPeriodReward},
    utilities::{SteakError, METADATA_SEED, STEAK_SEED},
};
use mpl_token_metadata::{state::Metadata, ID as METADATA_PROGRAM};

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
    pub reward_mint: Account<'info, Mint>,
    #[account(seeds=[METADATA_SEED,METADATA_PROGRAM.as_ref(),collection.key().as_ref()],bump,
    seeds::program=METADATA_PROGRAM)]
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
    increase_count: Option<u64>,
) -> Result<()> {
    let collection_metadata =
        try_from_slice_unchecked::<Metadata>(&ctx.accounts.collection_metadata.data.borrow())
            .unwrap();

    require!(
        collection_metadata.update_authority == ctx.accounts.update_authority.key(),
        SteakError::InvalidCollectionAuthority
    );

    let stake_config = &mut ctx.accounts.staking_config;

    require!(
        collection_metadata.data.symbol == collection_symbol,
        SteakError::InvalidCollectionSymbol
    );

    let stake_period = StakingPeriodReward::try_from_primitive(staking_period).unwrap();

    stake_config.staking_period = stake_period;
    stake_config.staking_authority = ctx.accounts.update_authority.key();
    stake_config.collection = ctx.accounts.collection.key();
    stake_config.staking_reward = staking_reward;

    stake_config.custom_staking_increase =
        CustomStakingIncrease::try_from_primitive(staking_increase).unwrap();

    stake_config.reward_mint = ctx.accounts.reward_mint.key();
    stake_config.increase_amount = increase_amount;
    stake_config.increase_count = increase_count;

    Ok(())
}
