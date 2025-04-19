use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, mint_to};

use crate::{
    state::{ProjectGrant, Tokenomics},
    utils::CustomError,
};

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
    let tokenomics = &mut ctx.accounts.tokenomics;

    // 1. Validar que no se excede el total asignado al grant
    let new_total = grant
        .total_claimed
        .checked_add(payload.amount)
        .ok_or(CustomError::Overflow)?;

    require!(
        new_total <= grant.total_allocated,
        CustomError::GrantExceeded
    );

    // 2. Validar que no se excede el supply global de rewards
    let mintable_amount = tokenomics
        .check_max_supply_exceeded(
            &tokenomics.max_rewards_supply(),
            &tokenomics.current_rewarded_supply,
            &payload.amount,
        )
        .ok_or(CustomError::RewardsSupplyExceeded)?;

    // 3. Actualizar estados
    grant.total_claimed = new_total;
    tokenomics.current_rewarded_supply = tokenomics
        .current_rewarded_supply
        .checked_add(mintable_amount)
        .unwrap();

    // 4. Mintear tokens
    let signer_seeds: &[&[u8]] = &[b"token", &[ctx.bumps.token_mint]];
    let signer = &[&signer_seeds[..]];

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
        mintable_amount,
    )?;

    Ok(())
}
