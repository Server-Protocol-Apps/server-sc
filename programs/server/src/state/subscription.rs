use anchor_lang::prelude::*;

#[account]
pub struct Subscription {
    pub bump: u8,            // 1
    pub last_claim: u128,    // 16
    pub total_claimed: u128, // 16
    pub subscribed_at: u128, // 16
    pub repo_pda: Pubkey,    // 32
    pub user_id: String,     // 4 + len
}

impl Subscription {
    pub fn size(user_id: &String) -> usize {
        8 + 1 + 16 + 16 + 16 + 32 + 4 + user_id.len()
    }

    pub fn initialize(&mut self, user_id: String, repo: Pubkey, timestamp: u128, bump: u8) {
        self.user_id = user_id;
        self.repo_pda = repo;
        self.bump = bump;
        self.total_claimed = 0;
        self.subscribed_at = timestamp;
        self.last_claim = timestamp;
    }

    pub fn update_total_claimed(&mut self, rewards: u128, timestamp: u128) {
        self.total_claimed = self.total_claimed.checked_add(rewards).unwrap();
        self.last_claim = timestamp;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Claim {
    pub commits: u64,
    pub timestamp: u128,
    pub user_id: String,
}

impl Claim {
    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }
}
