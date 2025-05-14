use anchor_lang::prelude::*;
use crate::state::TeamVesting;
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(payload: InitializeVestingPayload)]
pub struct InitializeTeamVesting<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [b"vesting", payload.team_wallet.as_ref()],
        bump,
        payer = admin,
        space = TeamVesting::SIZE,
    )]
    pub team_vesting: Account<'info, TeamVesting>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct InitializeVestingPayload {
    pub team_wallet: Pubkey,
    pub total_allocation: u64,
}

pub fn handler(ctx: Context<InitializeTeamVesting>, payload: InitializeVestingPayload) -> Result<()> {
    let vesting = &mut ctx.accounts.team_vesting;
    let now = ctx.accounts.clock.unix_timestamp;

    // ✅ WHITELIST DE WALLETS PERMITIDAS
    let allowed_wallets = vec![
        Pubkey::from_str("26zefaaRq7mcrkwPcssPALtJkoNGYSSBpWJTGxyqwiuo").unwrap(),
        Pubkey::from_str("Htk12Mz2YoJqwoK2xh2P8NJqznzjNbteKfvC7RyQogoZ").unwrap(),
    ];

    require!(
        allowed_wallets.contains(&payload.team_wallet),
        crate::utils::CustomError::Unauthorized
    );

    vesting.team_wallet = payload.team_wallet;
    vesting.total_allocation = payload.total_allocation;
    vesting.claimed = 0;
    vesting.start_time = now;

    // ⏳ TEMPORALMENTE: CLIFF DE 48 HORAS
    vesting.cliff_months = 0;
    vesting.total_months = 2;
    vesting.unlock_interval_months = 1;

    vesting.bump = ctx.bumps.team_vesting;

    Ok(())
}

