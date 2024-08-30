use crate::{state::Admin, utils::MPL_TOKEN_METADATA_ID};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::instructions::CreateV1CpiBuilder;

pub fn init(ctx: Context<InitToken>, payload: InitPayload) -> Result<()> {
    let seed = b"token";
    let bump = ctx.bumps.token_mint;
    let signer: &[&[&[u8]]] = &[&[seed, &[bump]]];

    ctx.accounts.admin_info.init(payload);

    CreateV1CpiBuilder::new(
        ctx.accounts
            .token_metadata_program
            .to_account_info()
            .as_ref(),
    )
    .authority(ctx.accounts.token_mint.to_account_info().as_ref())
    .mint(ctx.accounts.token_mint.to_account_info().as_ref(), false)
    .update_authority(ctx.accounts.token_mint.to_account_info().as_ref(), true)
    .system_program(ctx.accounts.system_program.to_account_info().as_ref())
    .payer(ctx.accounts.admin.to_account_info().as_ref())
    .name(String::from("Origin"))
    .uri(String::from("uri"))
    .is_mutable(true)
    .decimals(9)
    .symbol(String::from("ORIGIN"))
    .token_standard(mpl_token_metadata::types::TokenStandard::Fungible)
    .metadata(ctx.accounts.metadata.to_account_info().as_ref())
    .seller_fee_basis_points(0)
    .sysvar_instructions(ctx.accounts.rent.to_account_info().as_ref())
    .invoke_signed(signer)?;

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct InitPayload {
    pub signer: Pubkey,
    pub be: [u8; 64],
}

#[derive(Accounts)]
pub struct InitToken<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds=[b"token"],
        bump,
        payer=admin,
        mint::decimals=9,
        mint::authority=token_mint
    )]
    pub token_mint: Account<'info, Mint>,
    #[account(
      init,
      space = Admin::LEN,
      payer = admin,
      seeds = [b"ADMIN"],
      bump,
    )]
    pub admin_info: Account<'info, Admin>,
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: account constraint checked in account trait
    #[account(address = MPL_TOKEN_METADATA_ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}
