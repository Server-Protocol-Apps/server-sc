use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Mint, mint_to};

use crate::{state::{TeamVesting, Tokenomics}, utils::CustomError};

#[derive(Accounts)]
pub struct ClaimTeamTokens<'info> {
    #[account(mut)]
    pub team_wallet: Signer<'info>,

    #[account(mut, seeds = [b"token"], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"tokenomics"], bump)]
    pub tokenomics: Account<'info, Tokenomics>,

    #[account(
        mut,
        seeds = [b"vesting", team_wallet.key().as_ref()],
        bump = team_vesting.bump,
        has_one = team_wallet @ CustomError::Unauthorized,
    )]
    pub team_vesting: Account<'info, TeamVesting>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ClaimTeamTokens>) -> Result<()> {
    let vesting = &mut ctx.accounts.team_vesting;
    let now = ctx.accounts.clock.unix_timestamp;
    let amount = vesting.available_to_claim(now);

    require!(amount > 0, CustomError::NothingToClaim);

    let mintable = ctx
        .accounts
        .tokenomics
        .amount_to_mint_for_team(&amount)
        .ok_or(CustomError::MaxSupplyExceeded)?;

    vesting.claimed = vesting.claimed.checked_add(mintable).ok_or(CustomError::Overflow)?;

    let seeds: &[&[u8]] = &[b"token", &[ctx.bumps.token_mint]];
    let signer = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
                authority: ctx.accounts.token_mint.to_account_info(),
            },
            signer,
        ),
        mintable,
    )?;

    Ok(())
}