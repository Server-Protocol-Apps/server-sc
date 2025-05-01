use anchor_lang::prelude::*;

use crate::state::Repo;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeRepoPayload {
    pub repo_owner: String,
    pub repo_name: String,
    pub repo_branch: String,
    pub timestamp: u128,
}

#[derive(Accounts)]
#[instruction(payload: InitializeRepoPayload)]
pub struct InitializeRepoAccount<'info> {
    #[account(
        init,
        seeds = [b"repo", payload.repo_owner.as_bytes(), payload.repo_name.as_bytes(), payload.repo_branch.as_bytes()],
        bump,
        payer = publisher,
        space = Repo::size(&payload.repo_name, &payload.repo_owner, &payload.repo_branch)
    )]
    pub repo: Account<'info, Repo>,

    #[account(mut)]
    pub publisher: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeRepoAccount>, payload: InitializeRepoPayload) -> Result<()> {
    let repo = &mut ctx.accounts.repo;

    repo.owner = payload.repo_owner;
    repo.name = payload.repo_name;
    repo.branch = payload.repo_branch;
    repo.votes = 0;
    repo.publisher = ctx.accounts.publisher.key();
    repo.bump = ctx.bumps.repo;
    repo.total_claimed = 0;
    repo.proposed_timestamp = payload.timestamp;
    repo.approved = false;
    repo.approved_timestamp = 0;
    repo.subscribers = 0;

    Ok(())
}
