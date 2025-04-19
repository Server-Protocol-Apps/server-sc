use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum VoteType {
    Up,
    Down,
}

#[account]
pub struct Vote {
    pub bump: u8,
    pub vote_type: VoteType,
    pub timestamp: u128,
    pub repo_pda: Pubkey,
    pub user_id: String,
    pub weight: u64,
}

impl Vote {
    pub fn size(user_id: &String) -> usize {
        8 + 1 + 3 + 16 + 32 + 4 + user_id.len() + 8
    }
}
