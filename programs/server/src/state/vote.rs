use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum VoteType {
    Up,
    Down,
}

#[account]
pub struct Vote {
    pub bump: u8,            // 1
    pub vote_type: VoteType, // 3
    pub timestamp: u128,     //16
    pub repo_pda: Pubkey,    // 32
    pub user_id: String,     // 4 + len
}

impl Vote {
    pub fn size(user_id: &String) -> usize {
        8 + 1 + 3 + 16 + 32 + 4 + user_id.len()
    }
}
