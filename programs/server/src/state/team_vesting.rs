use anchor_lang::prelude::*;

#[account]
pub struct TeamVesting {
    pub team_wallet: Pubkey,
    pub start_time: i64,
    pub total_allocation: u64,
    pub claimed: u64,
    pub cliff_months: u8,
    pub total_months: u8,
    pub unlock_interval_months: u8,
    pub bump: u8,
}

impl TeamVesting {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1 + 1 + 1 + 1;

    pub fn available_to_claim(&self, current_time: i64) -> u64 {
        let cliff_end = self.start_time + (self.cliff_months as i64 * 30 * 24 * 60 * 60);
        if current_time < cliff_end {
            return 0;
        }

        let total_intervals = self.total_months / self.unlock_interval_months;
        let interval_seconds = self.unlock_interval_months as i64 * 30 * 24 * 60 * 60;

        let elapsed = current_time - cliff_end;
        let intervals_passed = (elapsed / interval_seconds).min(total_intervals as i64);

        let unlocked = (self.total_allocation / total_intervals as u64) * intervals_passed as u64;

        unlocked.saturating_sub(self.claimed)
    }
}