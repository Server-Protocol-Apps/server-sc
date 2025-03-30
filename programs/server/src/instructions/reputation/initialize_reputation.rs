use anchor_lang::prelude::*;
use crate::state::Reputation;

#[derive(Accounts)]
pub struct InitializeReputation<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"reputation", user.key().as_ref()],
        bump,
        space = Reputation::SIZE,
    )]
    pub reputation_account: Account<'info, Reputation>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeReputation>) -> Result<()> {
    let clock = Clock::get()?;
    let account = &mut ctx.accounts.reputation_account;

    account.user = ctx.accounts.user.key();
    account.reputation = 1;
    account.last_updated_ts = clock.unix_timestamp;
    account.last_gain_day = clock.unix_timestamp / (24 * 60 * 60);
    account.daily_gain_count = 0;
    account.bump = *ctx.bumps.get("reputation_account").unwrap();

    Ok(())
}