use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    state::{Admin, Claim, Repo, RepoPayload, Subscription},
    utils::{Coupon, CustomError},
};

pub fn claim_rewards(ctx: Context<ClaimRewards>, payload: ClaimRewardsPayload) -> Result<()> {
    payload
        .coupon
        .verify(&payload.claim.serialize(), &ctx.accounts.admin.be)?;
    require!(
        payload.claim.timestamp > ctx.accounts.subscription.last_claim,
        CustomError::ClaimedAlready
    );

    let seed = b"token";
    let bump = ctx.bumps.token;
    let signer: &[&[&[u8]]] = &[&[seed, &[bump]]];

    ctx.accounts
        .subscription
        .update_total_claimed(payload.claim.commits.into(), payload.claim.timestamp);

    ctx.accounts
        .repo
        .update_total_claimed(payload.claim.commits.into());

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.token.to_account_info(),
                mint: ctx.accounts.token.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
            },
            signer,
        ),
        payload.claim.commits.checked_mul(1000000000).unwrap(),
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ClaimRewardsPayload {
    pub repo: RepoPayload,
    pub claim: Claim,
    pub coupon: Coupon,
}

#[derive(Accounts)]
#[instruction(payload: ClaimRewardsPayload)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
        bump,
    )]
    pub repo: Account<'info, Repo>,
    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,
    #[account(
        mut,
        seeds = [b"sub", payload.claim.user_id.as_bytes() ,repo.key().as_ref()],
        bump,
    )]
    pub subscription: Account<'info, Subscription>,
    #[account(
      mut,
      seeds=[b"token"],
      bump,
      mint::authority=token
    )]
    pub token: Account<'info, Mint>,
    #[account(
      init_if_needed,
      payer=signer,
      associated_token::mint = token,
      associated_token::authority = signer,
    )]
    pub destination: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
