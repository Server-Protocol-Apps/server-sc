use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::state::StakingConfig;

#[derive(Accounts)]
pub struct InitializeStaking<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"vault_token_account"],
        bump,
        token::mint = mint,
        token::authority = vault_token_account
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        seeds = [b"staking_config"],
        bump,
        space = StakingConfig::SIZE,
    )]
    pub config: Account<'info, StakingConfig>,

    pub mint: Account<'info, Mint>, // $SERVER mint

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeStaking>) -> Result<()> {
    ctx.accounts.config.bytes_per_second_per_token = 1000; // == 0.000001
    ctx.accounts.config.bump = ctx.bumps.config;
    msg!("Staking vault initialized");
    Ok(())
}