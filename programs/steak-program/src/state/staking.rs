use anchor_lang::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[account]
pub struct StakingConfig {
    pub staking_authority: Pubkey,
    pub collection: Pubkey,
    pub staking_period: StakingPeriodReward,
    pub custom_staking_increase: CustomStakingIncrease,
    pub staking_reward: u64,
    pub fee_amount: u8,
    pub reward_mint: Pubkey,
    pub increase_amount: Option<u64>,
    pub increase_count: Option<u64>,
}

#[derive(AnchorDeserialize, AnchorSerialize, TryFromPrimitive, IntoPrimitive, Clone)]
#[repr(u8)]
pub enum StakingPeriodReward {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CustomStakingIncrease {
    None,
    Double,
    AddAmount,
    MultiplyAmount,
    AddPerNftAmount,
}
