use anchor_lang::prelude::*;

use crate::{utils::CustomError, InitPayload};

#[account]
pub struct Tokenomics {
    pub bump: u8,
    pub rewards_percentage: u8,
    pub team_percentage: u8,
    pub total_supply: u64,
    pub tokens_per_commit: u64,

    pub current_rewarded_supply: u64,
    pub current_team_supply: u64,
}

impl Tokenomics {
    pub fn size() -> usize {
        8 + 1 + 1 + 1 + 1 + 8 + 8 + 8 + 8
    }

    pub fn init(&mut self, bump: u8, payload: &InitPayload) {
        self.bump = bump;

        self.total_supply = payload.total_supply;
        self.rewards_percentage = payload.rewards_percentage;
        self.team_percentage = payload.team_percentage;
        self.tokens_per_commit = payload.tokens_per_commit;

        self.current_rewarded_supply = 0;
        self.current_team_supply = 0;
    }

    pub fn max_rewards_supply(&self) -> u64 {
        self.total_supply
            .checked_mul(self.rewards_percentage as u64)
            .unwrap()
            .checked_div(100)
            .unwrap()
    }

    pub fn max_team_supply(&self) -> u64 {
        self.total_supply
            .checked_mul(self.team_percentage as u64)
            .unwrap()
            .checked_div(100)
            .unwrap()
    }

    pub fn check_max_supply_exceeded(
        &self,
        max_supply: &u64,
        current_supply: &u64,
        amount_to_mint: &u64,
    ) -> Option<u64> {
        if current_supply >= max_supply {
            return None;
        }

        let new_supply = amount_to_mint.checked_add(*current_supply).unwrap();
        if new_supply > *max_supply {
            let diff = max_supply
                .checked_sub(self.current_rewarded_supply)
                .unwrap();
            msg!("diff {:?}", diff);
            return Some(diff);
        }

        Some(*amount_to_mint)
    }

    pub fn amount_to_mint_for_team(&mut self, amount: &u64) -> Option<u64> {
        let amount_to_mint = self.check_max_supply_exceeded(
            &self.max_team_supply(),
            &self.current_team_supply,
            amount,
        );

        if None == amount_to_mint {
            return None;
        }

        let unwrapped_amount_to_mint = amount_to_mint.unwrap();
        self.current_team_supply = self
            .current_team_supply
            .checked_add(unwrapped_amount_to_mint)
            .unwrap();

        Some(unwrapped_amount_to_mint)
    }

    pub fn amount_to_mint_for_reward(&mut self, commits: &u64) -> Option<u64> {
        let amount_to_mint = self.check_max_supply_exceeded(
            &self.max_rewards_supply(),
            &self.current_rewarded_supply,
            &commits.checked_mul(self.tokens_per_commit).unwrap(),
        );
        msg!("amount_to_mint {:?}", amount_to_mint);
        if amount_to_mint == None {
            return None;
        }
        let unwrapped_amount_to_mint = amount_to_mint.unwrap();
        self.current_rewarded_supply = self
            .current_rewarded_supply
            .checked_add(unwrapped_amount_to_mint)
            .unwrap();

        Some(unwrapped_amount_to_mint)
    }
}
