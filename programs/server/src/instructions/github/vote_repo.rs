use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, TokenAccount, Transfer, Mint};
use crate::{
    state::{Admin, BytesUsage, Repo, RepoPayload, Reputation, Vote, VoteType},
    utils::{calculate_internal_amount, CustomError, SERVER_DECIMALS},
};

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
        space = Vote::size(&payload.user_id),
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

    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,

    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<'_, '_, 'info, 'info, VoteRepo<'info>>, payload: VoteRepoPayload) -> Result<()> {
    let clock = Clock::get()?;

    // -------------------------
    // Recuperar remaining_accounts
    // -------------------------

    require!(
        ctx.remaining_accounts.len() >= 6,
        CustomError::InvalidRemainingAccounts
    );

    let user_bytes_ata = Account::<TokenAccount>::try_from(&ctx.remaining_accounts[0])?;
    let user_server_ata = Account::<TokenAccount>::try_from(&ctx.remaining_accounts[1])?;
    let treasury_server_ata = Account::<TokenAccount>::try_from(&ctx.remaining_accounts[2])?;
    let bytes_mint = Account::<Mint>::try_from(&ctx.remaining_accounts[3])?;
    let token_program = ctx.remaining_accounts[5].clone();

    let just_initialized = ctx.accounts.vote.timestamp == 0;

    // -------------------------
    // 1. Validar rango de peso de BYTES
    // -------------------------

    require!(
        payload.bytes_used >= 100 && payload.bytes_used <= 10_000,
        CustomError::InvalidVoteWeight
    );

    // -------------------------
    // 2. Quemar BYTES
    // -------------------------

    let burn_ctx = CpiContext::new(
        token_program.to_account_info(),
        Burn {
            mint: bytes_mint.to_account_info(),
            from: user_bytes_ata.to_account_info(),
            authority: ctx.accounts.voter.to_account_info(),
        },
    );
    token::burn(burn_ctx, payload.bytes_used)?;

    // -------------------------
    // 3. Transferir 5 SERVER como fee
    // -------------------------

    let server_fee = calculate_internal_amount(5, SERVER_DECIMALS);

    let transfer_ctx = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_server_ata.to_account_info(),
            to: treasury_server_ata.to_account_info(),
            authority: ctx.accounts.voter.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, server_fee)?;

    // -------------------------
    // 4. Actualizar reputación
    // -------------------------

    ctx.accounts.reputation.apply_decay(clock.unix_timestamp);
    ctx.accounts.reputation.update_after_bytes_use(clock.unix_timestamp);

    // -------------------------
    // 5. Calcular peso total del voto
    // -------------------------

    let vote_weight = match payload.vote_type {
        VoteType::Up => payload.bytes_used as i128 * ctx.accounts.reputation.reputation as i128,
        VoteType::Down => -(payload.bytes_used as i128 * ctx.accounts.reputation.reputation as i128),
    };

    // -------------------------
    // 6. Guardar o actualizar voto
    // -------------------------

    if just_initialized {
        ctx.accounts.vote.user_id = payload.user_id.clone();
        ctx.accounts.vote.vote_type = payload.vote_type;
        ctx.accounts.vote.repo_pda = ctx.accounts.repo.key();
        ctx.accounts.vote.bump = ctx.bumps.vote;
        ctx.accounts.vote.timestamp = payload.timestamp;
        ctx.accounts.vote.weight = payload.bytes_used;

        ctx.accounts.repo.vote(&ctx.accounts.vote);
    } else {
        require!(
            ctx.accounts.vote.vote_type != payload.vote_type,
            CustomError::VotedAlready
        );

        let previous_type = ctx.accounts.vote.vote_type.clone();
        let previous_weight = ctx.accounts.vote.weight;

        ctx.accounts.vote.vote_type = payload.vote_type;
        ctx.accounts.vote.timestamp = payload.timestamp;
        ctx.accounts.vote.weight = payload.bytes_used;

        let previous_vote_weight = match previous_type {
            VoteType::Up => previous_weight as i128 * ctx.accounts.reputation.reputation as i128,
            VoteType::Down => -(previous_weight as i128 * ctx.accounts.reputation.reputation as i128),
        };

        let net_change = vote_weight - previous_vote_weight;
        ctx.accounts.repo.change_vote(net_change, ctx.accounts.vote.timestamp);
    }

    // -------------------------
    // 7. Actualizar uso de BYTES
    // -------------------------

    ctx.accounts.bytes_usage.last_bytes_use_ts = clock.unix_timestamp;

    Ok(())
}
