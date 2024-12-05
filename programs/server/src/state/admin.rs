use anchor_lang::prelude::*;

use crate::{utils::CustomError, InitPayload};

#[account]
pub struct Admin {
    pub signer: Pubkey,
    pub be: [u8; 64],
    pub team_wallet: Pubkey,
}

impl Admin {
    pub const LEN: usize = 8 + 32 + 32 + 32 * 8;

    pub fn set_signer(&mut self, signer: Pubkey) {
        self.signer = signer;
    }
    pub fn set_be(&mut self, be: [u8; 64]) {
        self.be = be
    }
    pub fn set_team_wallet(&mut self, team_wallet: Pubkey) {
        self.team_wallet = team_wallet;
    }

    pub fn init(&mut self, init_payload: &InitPayload) {
        self.be = init_payload.be;
        self.signer = init_payload.signer;
        self.team_wallet = init_payload.team_wallet
    }

    pub fn require_admin(&self, payer: &Signer) -> Result<()> {
        require_eq!(payer.key(), self.signer, CustomError::AdminOnly);
        Ok(())
    }
}
