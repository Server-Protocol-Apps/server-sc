use anchor_lang::prelude::*;

use crate::{
    state::{Repo, RepoPayload, Subscription},
    utils::{Coupon, CustomError},
};

pub fn subscribe(ctx: Context<Subscribe>, payload: SubscribePayload) -> Result<()> {
    payload
        .coupon
        .verify(&payload.user_id.try_to_vec().unwrap())?;
    require!(ctx.accounts.repo.approved, CustomError::UnapprovedRepo);

    ctx.accounts.subscription.initialize(
        payload.user_id,
        ctx.accounts.repo.key(),
        payload.timestamp,
        ctx.bumps.subscription,
    );
    ctx.accounts.repo.add_subscriber();

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SubscribePayload {
    pub repo: RepoPayload,
    pub user_id: String,
    pub coupon: Coupon,
    pub timestamp: u128,
}

#[derive(Accounts)]
#[instruction(payload: SubscribePayload)]
pub struct Subscribe<'info> {
    #[account(
    mut,
    seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
    bump,
  )]
    pub repo: Account<'info, Repo>,
    #[account(
    init,
    seeds = [b"sub", payload.user_id.as_bytes(), repo.key().as_ref()],
    bump,
    payer=signer,
    space = Subscription::size(&payload.user_id)
  )]
    pub subscription: Account<'info, Subscription>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
