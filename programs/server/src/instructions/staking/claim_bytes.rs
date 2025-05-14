use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, mint_to};

use crate::state::{Staking, StakingConfig, BytesUsage};
use crate::utils::CustomError;

#[derive(Accounts)]
pub struct ClaimBytes<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"staking", staker.key().as_ref()],
        bump = staking_account.bump,
        has_one = staker
    )]
    pub staking_account: Account<'info, Staking>,

    #[account(
        seeds = [b"staking_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakingConfig>,

    #[account(
        mut,
        seeds = [b"bytes_usage", staker.key().as_ref()],
        bump = bytes_usage.bump,
    )]
    pub bytes_usage: Account<'info, BytesUsage>,

    #[account(
        mut
        // Ya no se deriva con seeds, se asume que es la cuenta de Mint correcta.
        // La autoridad se verifica a través de mint_authority PDA usada en mint_to.
        // seeds = [b"bytes_mint"],
        // bump
    )]
    pub bytes_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bytes_mint,
        associated_token::authority = staker
    )]
    pub user_bytes_ata: Account<'info, TokenAccount>,

    /// CHECK: This is a PDA mint authority derived from [b"bytes_mint"], verified in the instruction
    #[account(seeds = [b"bytes_mint"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ClaimBytes>) -> Result<()> {
    const SERVER_TOKEN_DECIMALS: u32 = 9; // Definir decimales del token $SERVER

    let staking = &mut ctx.accounts.staking_account;
    let config = &ctx.accounts.config;
    let bytes_usage = &ctx.accounts.bytes_usage;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    // Tiempo desde último claim
    // Usar checked_sub para evitar pánico si last_claim_ts > now, aunque require! lo cubre.
    let elapsed = now.checked_sub(staking.last_claim_ts)
        .ok_or_else(|| error!(CustomError::InvalidClaimTime))?; // O un error más específico si es negativo.
    require!(elapsed > 0, CustomError::InvalidClaimTime);

    // Convertir amount_staked (unidades mínimas de $SERVER) a unidades completas
    let amount_staked_minimal_u128 = staking.amount_staked as u128;
    let divisor_server_decimals = 10u128.checked_pow(SERVER_TOKEN_DECIMALS)
        .ok_or_else(|| error!(CustomError::Overflow))?; // Usar CustomError::Overflow
    
    require!(divisor_server_decimals > 0, CustomError::Overflow); // Usar CustomError::Overflow

    let amount_staked_full_units_u128 = amount_staked_minimal_u128
        .checked_div(divisor_server_decimals)
        .ok_or_else(|| error!(CustomError::Overflow))?;

    // Calcular emisión base (unidades mínimas de $BYTES)
    // base_amount = (full_$SERVER_staked) * (minimal_$BYTES_per_sec_per_full_$SERVER) * (elapsed_seconds)
    let base_amount_u128 = amount_staked_full_units_u128
        .checked_mul(config.bytes_per_second_per_token as u128)
        .ok_or_else(|| error!(CustomError::Overflow))?
        .checked_mul(elapsed as u128) // elapsed ya es i64, y se verificó > 0
        .ok_or_else(|| error!(CustomError::Overflow))?;

    // Cálculo de penalización
    // Usar unwrap_or(0) para since_last_use si now < last_bytes_use_ts para evitar pánico,
    // aunque la lógica de penalty_multiplier debería manejarlo.
    let since_last_use = now.checked_sub(bytes_usage.last_bytes_use_ts).unwrap_or(0); 
    let penalty_multiplier = calculate_penalty_multiplier(since_last_use); // u64 (0-100)

    // Aplicar penalización
    let final_amount_intermediate_u128 = base_amount_u128
        .checked_mul(penalty_multiplier as u128)
        .ok_or_else(|| error!(CustomError::Overflow))?;
    
    // Asegurarse de que el divisor (100) no sea cero, aunque es constante.
    let final_amount_u128 = final_amount_intermediate_u128
        .checked_div(100) 
        .ok_or_else(|| error!(CustomError::Overflow))?;

    // Convertir de forma segura a u64 para mint_to
    let amount_to_mint_u64 = u64::try_from(final_amount_u128)
        .map_err(|_| error!(CustomError::Overflow))?; // Usar CustomError::Overflow

    // Mint de BYTES
    let seeds: &[&[u8]] = &[b"bytes_mint", &[ctx.bumps.mint_authority]];
    let signer = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.bytes_mint.to_account_info(),
                to: ctx.accounts.user_bytes_ata.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer,
        ),
        amount_to_mint_u64,
    )?;

    // Actualización de timestamps
    staking.last_claim_ts = now;

    Ok(())
}

fn calculate_penalty_multiplier(since_last_use: i64) -> u64 {
    let week = 7 * 24 * 60 * 60;
    let month = 4 * week;

    if since_last_use < month {
        return 100;
    }

    let weeks_inactive = (since_last_use - month) / week;

    match weeks_inactive {
        0 => 90,
        1 => 80,
        2 => 70,
        3 => 60,
        4 => 45,
        5 => 30,
        6 => 15,
        _ => 0,
    }
}