use anchor_lang::prelude::{borsh::BorshSerialize, *};

use crate::{state::Admin, utils::Coupon};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct OffChainData {
    pub a: u8,
    pub b: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct VerifyCouponPayload {
    pub coupon: Coupon,
    pub data: OffChainData,
}

pub fn verify_coupon(_ctx: Context<VerifyCoupon>, payload: VerifyCouponPayload) -> Result<()> {
    let recovered_pubkey = payload
        .coupon
        .verify(&payload.data.try_to_vec().unwrap(), &_ctx.accounts.admin.be)?;

    msg!("Recovered pub key {:?}", recovered_pubkey);

    Ok(())
}

#[derive(Accounts)]
#[instruction(payload: VerifyCouponPayload)]
pub struct VerifyCoupon<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"ADMIN"],
        bump,
    )]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
}
