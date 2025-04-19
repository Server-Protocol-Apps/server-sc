use anchor_lang::prelude::*;
use crate::state::Admin;

#[derive(Accounts)]
pub struct SetTreasuryWallet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,
}

pub fn handler(ctx: Context<SetTreasuryWallet>, new_wallet: Pubkey) -> Result<()> {
    ctx.accounts.admin.require_admin(&ctx.accounts.payer)?;
    ctx.accounts.admin.set_treasury_wallet(new_wallet);
    Ok(())
}
