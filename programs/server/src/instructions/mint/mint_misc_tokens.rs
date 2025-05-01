use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use crate::state::Tokenomics;
use crate::utils::CustomError;

#[derive(Accounts)]
pub struct MintMiscTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token"],
        bump,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"tokenomics"],
        bump,
    )]
    pub tokenomics: Account<'info, Tokenomics>,

    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintMiscPayload {
    pub amount: u64,
}

pub fn handler(ctx: Context<MintMiscTokens>, payload: MintMiscPayload) -> Result<()> {
    let tokenomics = &mut ctx.accounts.tokenomics;

    let allocated = tokenomics
        .current_rewarded_supply
        .checked_add(tokenomics.current_team_supply)
        .ok_or(CustomError::Overflow)?;

    let max_misc = tokenomics
        .total_supply
        .checked_sub(allocated)
        .ok_or(CustomError::Overflow)?;

    let new_misc_total = tokenomics
        .current_misc_supply
        .checked_add(payload.amount)
        .ok_or(CustomError::Overflow)?;

    require!(new_misc_total <= max_misc, CustomError::ExceedsAvailableSupply);

    tokenomics.current_misc_supply = new_misc_total;

    // Mint tokens to destination
    let signer_seeds: &[&[u8]] = &[b"token", &[ctx.bumps.token_mint]];
    let signer = &[&signer_seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.token_mint.to_account_info(),
            },
            signer,
        ),
        payload.amount,
    )?;

    Ok(())
}
