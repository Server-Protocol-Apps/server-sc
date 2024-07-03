use anchor_lang::prelude::{borsh::BorshSerialize, *};

use crate::utils::Coupon;

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
    let recovered_pubkey = payload.coupon.verify(&payload.data.try_to_vec().unwrap())?;

    msg!("Recovered pub key {:?}", recovered_pubkey);

    Ok(())
}

#[derive(Accounts)]
#[instruction(payload: VerifyCouponPayload)]
pub struct VerifyCoupon<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
