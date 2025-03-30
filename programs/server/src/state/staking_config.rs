use anchor_lang::prelude::*;

#[account]
pub struct StakingConfig {
    pub bytes_per_second_per_token: u64,
    pub bump: u8,
}

impl StakingConfig {
    pub const SIZE: usize = 8 + 8 + 1; // anchor disc. + u64 + bump
}