use anchor_lang::prelude::*;

use crate::state::ProjectGrant;

#[derive(Accounts)]
#[instruction(repo_id: Pubkey)]
pub struct CreateGrant<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [b"grant", repo_id.as_ref()],
        bump,
        payer = admin,
        space = ProjectGrant::SIZE,
    )]
    pub project_grant: Account<'info, ProjectGrant>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateGrantPayload {
    pub repo_id: Pubkey,
    pub authority: Pubkey,
    pub total_allocated: u64,
}

pub fn handler(ctx: Context<CreateGrant>, payload: CreateGrantPayload) -> Result<()> {
    let grant = &mut ctx.accounts.project_grant;

    grant.repo_id = payload.repo_id;
    grant.authority = payload.authority;
    grant.total_allocated = payload.total_allocated;
    grant.total_claimed = 0;
    grant.grant_round = 1;
    grant.bump = ctx.bumps.project_grant;

    Ok(())
}