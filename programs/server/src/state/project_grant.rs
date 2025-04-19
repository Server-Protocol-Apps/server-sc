use anchor_lang::prelude::*;

#[account]
pub struct ProjectGrant {
    pub repo_id: Pubkey,
    pub authority: Pubkey,
    pub total_allocated: u64,
    pub total_claimed: u64,
    pub grant_round: u8,
    pub bump: u8,
}

impl ProjectGrant {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1; // 90 bytes
}