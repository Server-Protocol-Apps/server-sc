use anchor_lang::prelude::*;
use crate::state::{Admin, Tokenomics};

#[derive(Accounts)]
pub struct AdminResetTeamSupply<'info> {
    #[account(mut)]
    pub admin_signer: Signer<'info>,

    #[account(
        seeds = [b"ADMIN"],
        bump
    )]
    pub admin_info: Account<'info, Admin>,

    #[account(
        mut,
        seeds = [b"tokenomics"],
        bump
    )]
    pub tokenomics: Account<'info, Tokenomics>,
}

pub fn handler(ctx: Context<AdminResetTeamSupply>) -> Result<()> {
    // Verify that the signer is the admin authorized in the Admin account
    require!(ctx.accounts.admin_info.signer == ctx.accounts.admin_signer.key(), crate::utils::CustomError::Unauthorized);

    ctx.accounts.tokenomics.current_team_supply = 0;
    msg!("Admin reset current_team_supply to 0 in Tokenomics account");

    Ok(())
} 