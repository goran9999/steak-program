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

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
#[repr(u8)]
pub enum CustomStakingIncrease {
    None,
    Double,
    AddAmount(u64),
    MultiplyAmount(u64),
    AddPerNftAmount(u64),
}
