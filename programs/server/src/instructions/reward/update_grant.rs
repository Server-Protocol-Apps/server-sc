use anchor_lang::prelude::*;

use crate::state::ProjectGrant;

#[derive(Accounts)]
pub struct UpdateGrantAllocation<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"grant", project_grant.repo_id.as_ref()],
        bump = project_grant.bump,
    )]
    pub project_grant: Account<'info, ProjectGrant>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateGrantPayload {
    pub new_total_allocated: u64,
}

pub fn handler(ctx: Context<UpdateGrantAllocation>, payload: UpdateGrantPayload) -> Result<()> {
    let grant = &mut ctx.accounts.project_grant;

    require!(
        payload.new_total_allocated > grant.total_allocated,
        crate::utils::CustomError::InvalidGrantUpdate
    );

    grant.total_allocated = payload.new_total_allocated;
    grant.grant_round = grant.grant_round.saturating_add(1); // protección contra overflow

    Ok(())
}