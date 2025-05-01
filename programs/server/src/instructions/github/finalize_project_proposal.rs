use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, burn, Transfer, transfer, Token, TokenAccount};

use crate::{
    state::{Repo, BytesUsage, Admin, Reputation},
    utils::{calculate_internal_amount, SERVER_DECIMALS},
};

#[derive(Accounts)]
pub struct FinalizeProjectProposal<'info> {
    #[account(mut, has_one = publisher)]
    pub repo: Account<'info, Repo>,

    #[account(seeds = [b"ADMIN"], bump)]
    pub admin: Account<'info, Admin>,

    #[account(
        init_if_needed,
        payer = publisher,
        seeds = [b"bytes_usage", publisher.key().as_ref()],
        bump,
        space = BytesUsage::SIZE,
    )]
    pub bytes_usage: Account<'info, BytesUsage>,

    #[account(
        init_if_needed,
        payer = publisher,
        seeds = [b"reputation", publisher.key().as_ref()],
        bump,
        space = Reputation::SIZE,
    )]
    pub reputation: Account<'info, Reputation>,

    #[account(mut)]
    pub publisher: Signer<'info>,

    #[account(mut)]
    pub user_bytes_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_server_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_server_ata: Account<'info, TokenAccount>,

    #[account(address = crate::utils::BYTES_MINT)]
    pub bytes_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(address = crate::utils::SERVER_MINT)]
    pub server_mint: Account<'info, anchor_spl::token::Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeProjectProposal>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp as u128;

    // Quemar 1M BYTES
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.bytes_mint.to_account_info(),
                from: ctx.accounts.user_bytes_ata.to_account_info(),
                authority: ctx.accounts.publisher.to_account_info(),
            },
        ),
        1_000_000,
    )?;

    // Transferir 100 $SERVER
    let server_fee = calculate_internal_amount(100, SERVER_DECIMALS);
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_server_ata.to_account_info(),
                to: ctx.accounts.treasury_server_ata.to_account_info(),
                authority: ctx.accounts.publisher.to_account_info(),
            },
        ),
        server_fee,
    )?;

    // Actualizar reputación
    ctx.accounts.reputation.apply_decay(timestamp as i64);
    ctx.accounts.reputation.update_after_bytes_use(timestamp as i64);

    // Actualizar uso de BYTES
    ctx.accounts.bytes_usage.last_bytes_use_ts = timestamp as i64;

    Ok(())
}
