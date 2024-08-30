use anchor_lang::prelude::*;

use crate::{utils::CustomError, InitPayload};

#[account]
pub struct Admin {
    pub signer: Pubkey,
    pub be: [u8; 64],
}

impl Admin {
    pub const LEN: usize = 8 + 32 + 32 + 32 * 8;
    pub fn set_signer(&mut self, signer: Pubkey) {
        self.signer = signer;
    }
    pub fn set_be(&mut self, be: [u8; 64]) {
        self.be = be
    }

    pub fn init(&mut self, init_payload: InitPayload) {
        self.be = init_payload.be;
        self.signer = init_payload.signer
    }

    pub fn require_admin(&self, payer: &Signer) -> Result<()> {
        require_eq!(payer.key(), self.signer, CustomError::AdminOnly);
        Ok(())
    }
}
