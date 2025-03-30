use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Burn, burn, Transfer, transfer};

use crate::{
    state::{Repo, RepoPayload, Vote, VoteType, BytesUsage},
    utils::{CustomError, SERVER_DECIMALS},
};

pub fn vote_repo(ctx: Context<VoteRepo>, payload: VoteRepoPayload) -> Result<()> {
    let repo = &mut ctx.accounts.repo;
    let vote = &mut ctx.accounts.vote;
    let just_initialized = vote.timestamp == 0;

    // 1. Validar rango de $BYTES usados (100 - 10_000)
    require!(
        payload.bytes_used >= 100 && payload.bytes_used <= 10_000,
        CustomError::InvalidVoteWeight
    );

    // 2. Quemar $BYTES del usuario
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.bytes_mint.to_account_info(),
                from: ctx.accounts.user_bytes_ata.to_account_info(),
                authority: ctx.accounts.voter.to_account_info(),
            },
        ),
        payload.bytes_used,
    )?;

    // 3. Quemar 5 $SERVER como fee (ajustado a decimales)
    let server_fee_amount = 5 * 10u64.pow(SERVER_DECIMALS as u32);
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.server_mint.to_account_info(),
                from: ctx.accounts.user_server_ata.to_account_info(),
                authority: ctx.accounts.voter.to_account_info(),
            },
        ),
        server_fee_amount,
    )?;

    // 4. Aplicar el voto ponderado
    let vote_weight = payload.bytes_used as i128;
    if just_initialized {
        vote.user_id = payload.user_id.clone();
        vote.vote_type = payload.vote_type;
        vote.repo_pda = repo.key();
        vote.bump = ctx.bumps.vote;
        vote.timestamp = payload.timestamp;
        vote.weight = payload.bytes_used;
        repo.vote(&vote);
    } else {
        require!(
            vote.vote_type != payload.vote_type,
            CustomError::VotedAlready
        );
        let previous_type = vote.vote_type.clone();
        let previous_weight = vote.weight;

        vote.vote_type = payload.vote_type;
        vote.timestamp = payload.timestamp;
        vote.weight = payload.bytes_used;

        repo.change_vote(&vote, previous_type, previous_weight);
    }

    // 5. Actualizar uso de BYTES
    let clock = Clock::get()?;
    ctx.accounts.bytes_usage.last_bytes_use_ts = clock.unix_timestamp;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct VoteRepoPayload {
    pub repo: RepoPayload,
    pub timestamp: u128,
    pub user_id: String,
    pub vote_type: VoteType,
    pub bytes_used: u64,
}

#[derive(Accounts)]
#[instruction(payload: VoteRepoPayload)]
pub struct VoteRepo<'info> {
    #[account(
        mut,
        seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
        bump,
    )]
    pub repo: Account<'info, Repo>,

    #[account(
        init_if_needed,
        seeds = [b"vote", payload.user_id.as_bytes(), repo.key().as_ref()],
        bump,
        payer = voter,
        space = Vote::size(&payload.user_id)
    )]
    pub vote: Account<'info, Vote>,

    #[account(
        init_if_needed,
        payer = voter,
        seeds = [b"bytes_usage", voter.key().as_ref()],
        bump,
        space = BytesUsage::SIZE,
    )]
    pub bytes_usage: Account<'info, BytesUsage>,

    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(mut)]
    pub user_bytes_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_server_ata: Account<'info, TokenAccount>,

    #[account(address = crate::utils::BYTES_MINT)]
    pub bytes_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(address = crate::utils::SERVER_MINT)]
    pub server_mint: Account<'info, anchor_spl::token::Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
