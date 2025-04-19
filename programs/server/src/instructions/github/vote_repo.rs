use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Burn, burn, Transfer, transfer};

use crate::{
    state::{Admin, BytesUsage, Repo, RepoPayload, Reputation, Vote, VoteType},
    utils::{calculate_internal_amount, CustomError, SERVER_DECIMALS},
};

pub fn vote_repo(ctx: Context<VoteRepo>, payload: VoteRepoPayload) -> Result<()> {
    let repo = &mut ctx.accounts.repo;
    let vote = &mut ctx.accounts.vote;
    let reputation = &mut ctx.accounts.reputation;

    let just_initialized = vote.timestamp == 0;

    // 1. Validar rango de peso de BYTES
    require!(
        payload.bytes_used >= 100 && payload.bytes_used <= 10_000,
        CustomError::InvalidVoteWeight
    );

    // 2. Quemar BYTES
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

    // 3. Transferir 5 SERVER como fee a Treasury
    let server_fee = calculate_internal_amount(5, SERVER_DECIMALS);
    let treasury = ctx.accounts.admin.treasury_wallet;
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_server_ata.to_account_info(),
        to: ctx.accounts.treasury_server_ata.to_account_info(),
        authority: ctx.accounts.voter.to_account_info(),
    };

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        ),
        server_fee,
    )?;

    // 4. Reputación: aplicar decay y actualizar
    let clock = Clock::get()?;
    reputation.apply_decay(clock.unix_timestamp);
    reputation.update_after_bytes_use(clock.unix_timestamp);

    // 5. Calcular peso total del voto
    let vote_weight = match payload.vote_type {
        VoteType::Up => payload.bytes_used as i128 * reputation.reputation as i128,
        VoteType::Down => -(payload.bytes_used as i128 * reputation.reputation as i128),
    };

    // 6. Guardar o actualizar voto
    if just_initialized {
        vote.user_id = payload.user_id.clone();
        vote.vote_type = payload.vote_type;
        vote.repo_pda = repo.key();
        vote.bump = ctx.bumps.vote;
        vote.timestamp = payload.timestamp;
        vote.weight = payload.bytes_used;
        repo.vote(vote);
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

        let previous_vote_weight = match previous_type {
            VoteType::Up => previous_weight as i128 * reputation.reputation as i128,
            VoteType::Down => -(previous_weight as i128 * reputation.reputation as i128),
        };

        let net_change = vote_weight - previous_vote_weight;
        repo.change_vote(net_change, vote.timestamp);
    }

    // 7. Actualizar uso de BYTES
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

    #[account(
        init_if_needed,
        payer = voter,
        seeds = [b"reputation", voter.key().as_ref()],
        bump,
        space = Reputation::SIZE,
    )]
    pub reputation: Account<'info, Reputation>,
    

    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(mut)]
    pub user_bytes_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_server_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,

    #[account(
        mut,
        associated_token::mint = server_mint,
        associated_token::authority = admin.treasury_wallet
    )]
    pub treasury_server_ata: Account<'info, TokenAccount>,

    #[account(address = crate::utils::BYTES_MINT)]
    pub bytes_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(address = crate::utils::SERVER_MINT)]
    pub server_mint: Account<'info, anchor_spl::token::Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
