use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, mint_to};

use crate::{state::{ProjectGrant, Tokenomics}, utils::CustomError};

#[derive(Accounts)]
pub struct MintRewardFromGrant<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

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

    #[account(
        mut,
        seeds = [b"grant", project_grant.repo_id.as_ref()],
        bump = project_grant.bump,
        has_one = authority @ CustomError::Unauthorized,
    )]
    pub project_grant: Account<'info, ProjectGrant>,

    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintFromGrantPayload {
    pub amount: u64,
}

pub fn handler(ctx: Context<MintRewardFromGrant>, payload: MintFromGrantPayload) -> Result<()> {
    let grant = &mut ctx.accounts.project_grant;

    let new_total = grant
        .total_claimed
        .checked_add(payload.amount)
        .ok_or(CustomError::Overflow)?;

    require!(
        new_total <= grant.total_allocated,
        CustomError::GrantExceeded
    );

    // Actualizamos claim
    grant.total_claimed = new_total;

    // Validamos que estamos dentro del límite de supply de rewards
    let mintable_amount = ctx
        .accounts
        .tokenomics
        .amount_to_mint_for_reward(payload.amount)
        .ok_or(CustomError::RewardsSupplyExceeded)?;

    // Minteamos
    let cpi_accounts = MintTo {
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
        authority: ctx.accounts.token_mint.to_account_info(),
    };

    let signer_seeds = &[b"token", &[ctx.bumps.token_mint]];
    let signer = &[&signer_seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        ),
        mintable_amount,
    )?;

    Ok(())
}