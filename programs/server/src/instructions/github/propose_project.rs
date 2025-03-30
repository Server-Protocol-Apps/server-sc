use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, burn, Token, TokenAccount};

use crate::{
    state::{Repo, RepoPayload, BytesUsage, Admin},
    utils::{Coupon, CustomError, BYTES_MINT, SERVER_MINT, SERVER_DECIMALS},
};

pub fn handler(ctx: Context<ProposeProject>, payload: ProposeProjectPayload) -> Result<()> {
    // 1. Verificar el cupón
    payload.coupon.verify(&payload.repo.serialize(), &ctx.accounts.admin.be)?;

    // 2. Validar el mínimo de BYTES a quemar
    require!(
        payload.bytes_used == 1_000_000,
        CustomError::InvalidProposalAmount
    );

    // 3. Quemar 1M BYTES
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.bytes_mint.to_account_info(),
                from: ctx.accounts.user_bytes_ata.to_account_info(),
                authority: ctx.accounts.publisher.to_account_info(),
            },
        ),
        payload.bytes_used,
    )?;

    // 4. Quemar 100 $SERVER
    let server_amount = 100 * 10u64.pow(SERVER_DECIMALS as u32);
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.server_mint.to_account_info(),
                from: ctx.accounts.user_server_ata.to_account_info(),
                authority: ctx.accounts.publisher.to_account_info(),
            },
        ),
        server_amount,
    )?;

    // 5. Crear el Repo
    let repo = &mut ctx.accounts.repo;
    repo.owner = payload.repo.owner;
    repo.name = payload.repo.name;
    repo.branch = payload.repo.branch;
    repo.votes = 0;
    repo.publisher = ctx.accounts.publisher.key();
    repo.bump = ctx.bumps.repo;
    repo.total_claimed = 0;
    repo.proposed_timestamp = payload.timestamp;
    repo.approved = false;
    repo.approved_timestamp = 0;
    repo.subscribers = 0;

    // 6. Actualizar uso de BYTES
    ctx.accounts.bytes_usage.last_bytes_use_ts = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ProposeProjectPayload {
    pub repo: RepoPayload,
    pub timestamp: u128,
    pub bytes_used: u64,
    pub coupon: Coupon,
}

#[derive(Accounts)]
#[instruction(payload: ProposeProjectPayload)]
pub struct ProposeProject<'info> {
    #[account(
        init,
        seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
        bump,
        payer = publisher,
        space = Repo::size(&payload.repo.name, &payload.repo.owner, &payload.repo.branch)
    )]
    pub repo: Account<'info, Repo>,

    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,

    #[account(
        init_if_needed,
        payer = publisher,
        seeds = [b"bytes_usage", publisher.key().as_ref()],
        bump,
        space = BytesUsage::SIZE,
    )]
    pub bytes_usage: Account<'info, BytesUsage>,

    #[account(mut)]
    pub publisher: Signer<'info>,

    #[account(mut)]
    pub user_bytes_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_server_ata: Account<'info, TokenAccount>,

    #[account(address = BYTES_MINT)]
    pub bytes_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(address = SERVER_MINT)]
    pub server_mint: Account<'info, anchor_spl::token::Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
