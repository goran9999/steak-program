use anchor_lang::{
    prelude::*,
    system_program::{self},
};
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{
    state::{DepositRewardEvent, StakingConfig},
    utilities::{SteakError, REWARD_SEED, STEAK_SEED},
};

#[derive(Accounts)]
#[instruction(collection_symbol:String)]
pub struct DepositReward<'info> {
    #[account(seeds=[collection_symbol.as_bytes(),staking_config.collection.as_ref(),
    update_authority.key.as_ref(),STEAK_SEED],bump)]
    pub staking_config: Account<'info, StakingConfig>,
    #[account(mut)]
    pub update_authority: Signer<'info>,
    #[account(address=staking_config.reward_mint @SteakError::WrongRewardMint)]
    pub reward_mint: Account<'info, Mint>,
    #[account(init_if_needed,seeds=[collection_symbol.as_bytes(),reward_mint.key().as_ref(),REWARD_SEED],bump,
    payer=update_authority,token::mint=reward_mint,token::authority=reward_tokens)]
    pub reward_tokens: Account<'info, TokenAccount>,
    #[account(constraint=source_tokens.mint==reward_mint.key() @SteakError::WrongRewardMint)]
    pub source_tokens: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn deposit_reward(ctx: Context<DepositReward>, deposit_amount: u64) -> Result<()> {
    let source_tokens = &ctx.accounts.source_tokens;

    if source_tokens.is_native() {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.update_authority.to_account_info(),
                    to: ctx.accounts.reward_tokens.to_account_info(),
                },
            ),
            deposit_amount,
        )?;
    } else {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    authority: ctx.accounts.update_authority.to_account_info(),
                    from: ctx.accounts.source_tokens.to_account_info(),
                    to: ctx.accounts.reward_tokens.to_account_info(),
                },
            ),
            deposit_amount,
        )?;
    }

    emit!(DepositRewardEvent {
        deposit_amount,
        deposit_timestamp: Clock::get().unwrap().unix_timestamp,
        deposit_mint: ctx.accounts.reward_mint.key()
    });

    Ok(())
}
