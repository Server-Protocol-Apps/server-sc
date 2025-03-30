use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount, InitializeAccount, initialize_account};

#[derive(Accounts)]
pub struct InitializeStaking<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: No constraints needed, just initializing
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"vault_token_account"],
        bump,
        token::mint = mint,
        token::authority = vault_authority
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
    ctx.accounts.config.bytes_per_second_per_token = 100; // == 0.0000001
    ctx.accounts.config.bump = *ctx.bumps.get("config").unwrap();
    msg!("Staking vault initialized");
    Ok(())
}