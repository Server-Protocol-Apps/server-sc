use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, transfer, Token, TokenAccount};

use crate::state::{Staking, BytesUsage};

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_token_account"],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = staker,
        seeds = [b"staking", staker.key().as_ref()],
        bump,
        space = Staking::SIZE,
    )]
    pub staking_account: Account<'info, Staking>,

    #[account(
        init_if_needed,
        payer = staker,
        seeds = [b"bytes_usage", staker.key().as_ref()],
        bump,
        space = BytesUsage::SIZE,
    )]
    pub bytes_usage: Account<'info, BytesUsage>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let staking = &mut ctx.accounts.staking_account;
    let bytes_usage = &mut ctx.accounts.bytes_usage;

    if staking.amount_staked == 0 {
        staking.start_ts = clock.unix_timestamp;
        staking.last_claim_ts = clock.unix_timestamp;
    }

    staking.staker = ctx.accounts.staker.key();
    staking.amount_staked = staking
        .amount_staked
        .checked_add(amount)
        .ok_or(ProgramError::InvalidArgument)?;
    staking.bump = ctx.bumps.staking_account;

    if bytes_usage.last_bytes_use_ts == 0 {
        bytes_usage.user = ctx.accounts.staker.key();
        bytes_usage.last_bytes_use_ts = clock.unix_timestamp;
        bytes_usage.bump = ctx.bumps.bytes_usage;
    }

    let vault_seeds: &[&[u8]] = &[b"vault_token_account", &[ctx.bumps.vault]];
    let signer = &[&vault_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.staker_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.staker.to_account_info(),
        },
        signer,
    );
    transfer(cpi_ctx, amount)?;

    Ok(())
}
