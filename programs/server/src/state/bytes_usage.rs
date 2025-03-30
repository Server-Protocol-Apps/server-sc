use anchor_lang::prelude::*;

#[account]
pub struct BytesUsage {
    pub user: Pubkey,
    pub last_bytes_use_ts: i64,
    pub bump: u8,
}

impl BytesUsage {
    pub const SIZE: usize = 8 + 32 + 8 + 1;
}