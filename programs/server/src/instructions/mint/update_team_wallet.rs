use anchor_lang::prelude::*;
use crate::state::{Admin, TeamVesting};

#[derive(Accounts)]
pub struct UpdateTeamWallet<'info> {
    #[account(mut)]
    pub admin_signer: Signer<'info>,

    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,

    #[account(
        mut,
        seeds = [b"vesting"],
        bump = team_vesting.bump,
    )]
    pub team_vesting: Account<'info, TeamVesting>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateTeamWalletPayload {
    pub new_team_wallet: Pubkey,
}

pub fn handler(ctx: Context<UpdateTeamWallet>, payload: UpdateTeamWalletPayload) -> Result<()> {
    // Verificamos que el signer sea el admin autorizado
    ctx.accounts.admin.require_admin(&ctx.accounts.admin_signer)?;

    ctx.accounts.team_vesting.team_wallet = payload.new_team_wallet;
    Ok(())
}