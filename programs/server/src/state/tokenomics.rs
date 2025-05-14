use anchor_lang::prelude::*;

use crate::InitPayload;

#[account]
pub struct Tokenomics {
    pub bump: u8,
    pub rewards_percentage: u8,
    pub team_percentage: u8,
    pub total_supply: u64,

    pub current_rewarded_supply: u64,
    pub current_team_supply: u64,
    pub current_misc_supply: u64,
}

impl Tokenomics {
    pub fn size() -> usize {
        8 + 1 + 1 + 1 + 8 + 8 + 8 + 8
    }

    pub fn init(&mut self, bump: u8, payload: &InitPayload) {
        self.bump = bump;

        self.total_supply = payload.total_supply;
        self.rewards_percentage = payload.rewards_percentage;
        self.team_percentage = payload.team_percentage;

        self.current_rewarded_supply = 0;
        self.current_team_supply = 0;
        self.current_misc_supply = 0;
    }

    pub fn max_rewards_supply(&self) -> u64 {
        self.total_supply
            .checked_div(100)
            .unwrap()
            .checked_mul(self.rewards_percentage as u64)
            .unwrap()
    }

    pub fn max_team_supply(&self) -> u64 {
        self.total_supply
            .checked_div(100)
            .unwrap()
            .checked_mul(self.team_percentage as u64)
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
                .checked_sub(*current_supply)
                .unwrap();
            return Some(diff);
        }

        Some(*amount_to_mint)
    }

    pub fn amount_to_mint_for_team(&mut self, amount: &u64) -> Option<u64> {
        let max_team_s = self.max_team_supply();

        let amount_to_mint = self.check_max_supply_exceeded(
            &max_team_s,
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
}
