use crate::{
    state::{Repo, RepoPayload, Vote, VoteType},
    utils::CustomError,
};
use anchor_lang::prelude::*;

pub fn vote_repo(ctx: Context<VoteRepo>, payload: VoteRepoPayload) -> Result<()> {
    let repo: &mut Account<'_, Repo> = &mut ctx.accounts.repo;
    let vote = &mut ctx.accounts.vote;
    let just_initialized = vote.timestamp == 0;

    require!(
        just_initialized || vote.vote_type != payload.vote_type,
        CustomError::VotedAlready
    );

    if just_initialized {
        vote.user_id = payload.user_id;
        vote.vote_type = payload.vote_type;
        vote.repo_pda = repo.key();
        vote.bump = ctx.bumps.vote;
        vote.timestamp = payload.timestamp;
        repo.vote(&vote);
    } else {
        vote.vote_type = payload.vote_type;
        vote.timestamp = payload.timestamp;
        repo.change_vote(&vote);
    }

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct VoteRepoPayload {
    pub repo: RepoPayload,
    pub timestamp: u128,
    pub user_id: String,
    pub vote_type: VoteType,
}

#[derive(Accounts)]
#[instruction(payload: VoteRepoPayload)]
pub struct VoteRepo<'info> {
    #[account(
        mut,
        seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
        bump,
    )]
    pub repo: Account<'info, Repo>,
    #[account(
        init_if_needed,
        seeds = [b"vote", payload.user_id.as_bytes(), repo.key().as_ref()],
        bump,
        payer=voter,
        space = Vote::size(&payload.user_id)
    )]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}
