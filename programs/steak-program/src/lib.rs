use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod utilities;
use instructions::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod steak_program {
    use anchor_lang::{prelude::Context, Result};

    use crate::instructions::{self, InitStaking};

    pub fn init_staking_config(
        ctx: Context<InitStaking>,
        collection_symbol: String,
        staking_period: u8,
        staking_reward: u64,
        staking_increase: u8,
        increase_amount: Option<u64>,
        increase_count: Option<u64>,
    ) -> Result<()> {
        instructions::init_staking_config(
            ctx,
            collection_symbol,
            staking_period,
            staking_reward,
            staking_increase,
            increase_amount,
            increase_count,
        )
    }
}
