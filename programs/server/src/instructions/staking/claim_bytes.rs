use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, mint_to};

use crate::state::{Staking, StakingConfig, BytesUsage};
use crate::utils::CustomError;

#[derive(Accounts)]
pub struct ClaimBytes<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"staking", staker.key().as_ref()],
        bump = staking_account.bump,
        has_one = staker
    )]
    pub staking_account: Account<'info, Staking>,

    #[account(
        seeds = [b"staking_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakingConfig>,

    #[account(
        mut,
        seeds = [b"bytes_usage", staker.key().as_ref()],
        bump = bytes_usage.bump,
    )]
    pub bytes_usage: Account<'info, BytesUsage>,

    #[account(
        mut
        // Ya no se deriva con seeds, se asume que es la cuenta de Mint correcta.
        // La autoridad se verifica a través de mint_authority PDA usada en mint_to.
        // seeds = [b"bytes_mint"],
        // bump
    )]
    pub bytes_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bytes_mint,
        associated_token::authority = staker
    )]
    pub user_bytes_ata: Account<'info, TokenAccount>,

    /// CHECK: This is a PDA mint authority derived from [b"bytes_mint"], verified in the instruction
    #[account(seeds = [b"bytes_mint"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ClaimBytes>) -> Result<()> {
    let staking = &mut ctx.accounts.staking_account;
    let config = &ctx.accounts.config;
    let bytes_usage = &ctx.accounts.bytes_usage;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    // Tiempo desde último claim
    let elapsed = now - staking.last_claim_ts;
    require!(elapsed > 0, CustomError::InvalidClaimTime);

    // Emisión base (sin penalización)
    let base_amount: u128 = staking.amount_staked as u128
        * config.bytes_per_second_per_token as u128
        * elapsed as u128;

    // Cálculo de penalización
    let since_last_use = now - bytes_usage.last_bytes_use_ts;
    let penalty_multiplier = calculate_penalty_multiplier(since_last_use);

    let final_amount = (base_amount * penalty_multiplier as u128) / 100;

    // Mint de BYTES
    let seeds: &[&[u8]] = &[b"bytes_mint", &[ctx.bumps.mint_authority]];
    let signer = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.bytes_mint.to_account_info(),
                to: ctx.accounts.user_bytes_ata.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer,
        ),
        final_amount as u64,
    )?;

    // Actualización de timestamps
    staking.last_claim_ts = now;

    Ok(())
}

fn calculate_penalty_multiplier(since_last_use: i64) -> u64 {
    let week = 7 * 24 * 60 * 60;
    let month = 4 * week;

    if since_last_use < month {
        return 100;
    }

    let weeks_inactive = (since_last_use - month) / week;

    match weeks_inactive {
        0 => 90,
        1 => 80,
        2 => 70,
        3 => 60,
        4 => 45,
        5 => 30,
        6 => 15,
        _ => 0,
    }
}