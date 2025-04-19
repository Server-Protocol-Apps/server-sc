use anchor_lang::prelude::*;

use crate::instructions::InitPayload;
#[account]
pub struct Admin {
    pub signer: Pubkey,
    pub be: [u8; 64],
    pub team_wallet: Pubkey,
    pub treasury_wallet: Pubkey,
}

impl Admin {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32; // actualizado (discriminator + 4 pubkeys)

    pub fn set_signer(&mut self, signer: Pubkey) {
        self.signer = signer;
    }

    pub fn set_be(&mut self, be: [u8; 64]) {
        self.be = be;
    }

    pub fn set_team_wallet(&mut self, team_wallet: Pubkey) {
        self.team_wallet = team_wallet;
    }

    pub fn set_treasury_wallet(&mut self, treasury_wallet: Pubkey) {
        self.treasury_wallet = treasury_wallet;
    }

    pub fn init(&mut self, init_payload: &InitPayload) {
        self.be = init_payload.be;
        self.signer = init_payload.signer;
        self.team_wallet = init_payload.team_wallet;
        self.treasury_wallet = init_payload.treasury_wallet; // 🆕
    }

    pub fn require_admin(&self, payer: &Signer) -> Result<()> {
        require_eq!(payer.key(), self.signer, crate::utils::CustomError::AdminOnly);
        Ok(())
    }
}
