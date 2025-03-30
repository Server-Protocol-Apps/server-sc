use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::state::Staking;

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(mut,
        seeds = [b"staking", staker.key().as_ref()],
        bump = staking_account.bump,
        has_one = staker
    )]
    pub staking_account: Account<'info, Staking>,

    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>, // Donde se guardan los tokens stakeados

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
    let staking = &mut ctx.accounts.staking_account;

    require!(amount <= staking.amount_staked, ProgramError::InsufficientFunds);

    staking.amount_staked = staking
        .amount_staked
        .checked_sub(amount)
        .ok_or(ProgramError::InvalidArgument)?;

    let vault_seeds = &[b"vault", &[ctx.bumps.get("vault").copied().unwrap_or_default()]];
    let signer = &[&vault_seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.staker_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            signer,
        ),
        amount,
    )?;

    Ok(())
}