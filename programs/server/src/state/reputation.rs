use anchor_lang::prelude::*;

#[account]
pub struct Reputation {
    pub user: Pubkey,
    pub reputation: u64,
    pub last_updated_ts: i64,
    pub last_gain_day: i64,
    pub daily_gain_count: u8,
    pub bump: u8,
}

impl Reputation {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 1;

    pub fn apply_decay(&mut self, now: i64) {
        let week = 7 * 24 * 60 * 60;
        let elapsed = now - self.last_updated_ts;

        if elapsed < week {
            return;
        }

        let weeks_inactive = elapsed / week;
        self.reputation = self
            .reputation
            .saturating_sub(weeks_inactive as u64)
            .max(1); // nunca menos de 1

        self.last_updated_ts = now;
    }

    pub fn update_after_bytes_use(&mut self, now: i64) {
        let current_day = now / (24 * 60 * 60);

        if current_day == self.last_gain_day {
            if self.daily_gain_count >= 3 {
                return; // ya alcanzó el límite de ganancia diaria
            }
            self.daily_gain_count += 1;
        } else {
            self.last_gain_day = current_day;
            self.daily_gain_count = 1;
        }

        if self.reputation < 20 {
            self.reputation += 1;
        }

        self.last_updated_ts = now;
    }
}
