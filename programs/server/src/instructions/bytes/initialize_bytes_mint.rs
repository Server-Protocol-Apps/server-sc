use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};


#[derive(Accounts)]
pub struct InitializeBytesMint<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: PDA que actuará como mint authority
    #[account(
        seeds = [b"bytes_mint"],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,

    /// Verificamos que aún no existe ninguna cuenta en esta dirección
    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub bytes_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeBytesMint>) -> Result<()> {
    msg!("✅ Mint de $BYTES creado exitosamente");
    Ok(())
}
