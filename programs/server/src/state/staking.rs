use anchor_lang::prelude::*;

#[account]
pub struct Staking {
    pub staker: Pubkey,
    pub amount_staked: u64,
    pub last_claim_ts: i64,
    pub start_ts: i64,
    pub bump: u8,
    pub last_bytes_use_ts: i64,
}

impl Staking {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 8; // = 73 bytes
}