use anchor_lang::prelude::*;

use crate::state::Admin;

pub fn set_signer(ctx: Context<ManageAdmin>, signer: Pubkey) -> Result<()> {
    ctx.accounts.admin.set_signer(signer);
    Ok(())
}

pub fn set_be(ctx: Context<ManageAdmin>, be: [u8; 64]) -> Result<()> {
    ctx.accounts.admin.set_be(be);
    Ok(())
}

#[derive(Accounts)]
pub struct ManageAdmin<'info> {
    #[account(mut, constraint = payer.key() == admin.signer.key())]
    pub payer: Signer<'info>,
    #[account(
      seeds = [b"ADMIN"],
      bump,
    )]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
}
