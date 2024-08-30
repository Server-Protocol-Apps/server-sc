use crate::{state::{Admin, Repo, RepoPayload}, utils::Coupon};
use anchor_lang::prelude::*;

pub fn add_repo(
        ctx: Context<AddRepo>,
        payload: AddRepoPayload,
    )-> Result<()> {
        payload.coupon.verify(&payload.repo.serialize(), &ctx.accounts.admin.be)?;

        let publisher = ctx.accounts.publisher.key();
        let repo = &mut ctx.accounts.repo;
        repo.owner = payload.repo.owner;
        repo.name = payload.repo.name;
        repo.branch = payload.repo.branch;
        repo.votes = 0;
        repo.publisher = publisher;
        repo.bump = ctx.bumps.repo;
        repo.total_claimed = 0;
        repo.proposed_timestamp = payload.timestamp;
        msg!("Repo {:?}/{:?} saved", repo.owner, repo.name);

        Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct AddRepoPayload {
    pub repo: RepoPayload,
    pub timestamp: u128,
    pub coupon: Coupon,
}

#[derive(Accounts)]
#[instruction(payload: AddRepoPayload)]
pub struct AddRepo<'info> {
    #[account(
        init,
        seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
        bump,
        payer = publisher,
        space = Repo::size(&payload.repo.name, &payload.repo.owner, &payload.repo.branch) 
    )]
    pub repo: Account<'info, Repo>,
    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,
    #[account(mut)]
    pub publisher: Signer<'info>,
    pub system_program: Program<'info, System>,
}
