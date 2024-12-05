use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    state::{Admin, Tokenomics},
    utils::{calculate_internal_amount, CustomError},
};

pub fn send_to_team(ctx: Context<MintToTeam>, amount: u64) -> Result<()> {
    let amount_to_mint = ctx.accounts.tokenomics.amount_to_mint_for_team(&amount);
    require!(amount_to_mint != None, CustomError::MaxSupplyExceeded);
    let unwrapped_amount_to_mint = amount_to_mint.unwrap();

    let seed = b"token";
    let bump = ctx.bumps.token;
    let signer: &[&[&[u8]]] = &[&[seed, &[bump]]];
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.token.to_account_info(),
                mint: ctx.accounts.token.to_account_info(),
                to: ctx.accounts.team_account_ata.to_account_info(),
            },
            signer,
        ),
        calculate_internal_amount(unwrapped_amount_to_mint, ctx.accounts.token.decimals),
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct MintToTeam<'info> {
    #[account(mut, constraint = payer.key() == admin.signer.key())]
    pub payer: Signer<'info>,
    #[account(
      seeds = [b"ADMIN"],
      bump,
    )]
    pub admin: Account<'info, Admin>,
    #[account(mut, constraint = team_account.key() == admin.team_wallet.key())]
    pub team_account: AccountInfo<'info>,
    #[account(
        seeds=[b"tokenomics"],
        bump,
    )]
    pub tokenomics: Account<'info, Tokenomics>,
    #[account(
      init_if_needed,
      payer=payer,
      associated_token::mint = token,
      associated_token::authority = team_account,
    )]
    pub team_account_ata: Account<'info, TokenAccount>,
    #[account(
      mut,
      seeds=[b"token"],
      bump,
      mint::authority=token
    )]
    pub token: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
