use anchor_lang::prelude::*;
use crate::state::{Admin, StakingConfig};
use crate::utils::CustomError;

#[derive(Accounts)]
pub struct SetStakingRate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,

    #[account(
        mut,
        seeds = [b"staking_config"],
        bump,
    )]
    pub config: Account<'info, StakingConfig>,
}

pub fn handler(ctx: Context<SetStakingRate>, new_rate: u64) -> Result<()> {
    ctx.accounts.admin.require_admin(&ctx.accounts.signer)?;
    ctx.accounts.config.bytes_per_second_per_token = new_rate;
    Ok(())
}