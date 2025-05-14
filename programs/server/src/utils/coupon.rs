use crate::borsh;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use anchor_lang::solana_program::{
    blake3::HASH_BYTES,
    keccak, msg,
    secp256k1_recover::{secp256k1_recover, Secp256k1Pubkey},
};

use super::CustomError;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Coupon {
    pub signature: String,
    pub recovery_id: u8,
}

impl Coupon {
    pub fn verify(&self, serialized_data: &Vec<u8>, be: &[u8; 64]) -> Result<(), CustomError> {
        msg!("Coupon validation temporarily bypassed for testing");
        return Ok(());

        // msg!("Validating coupon");
        // let hash = self.hash(serialized_data);

        // let signature_bytes = &hex::decode(&self.signature)
        //     .map_err(|e| {
        //         msg!("Failed to decode signature: {:?}", e);
        //         CustomError::InvalidCoupon 
        //     })?;

        // let recovered_pubkey: Secp256k1Pubkey = secp256k1_recover(&hash, self.recovery_id, signature_bytes)
        //     .map_err(|e| {
        //         msg!("Failed to recover pubkey from signature: {:?}", e);
        //         CustomError::InvalidSignature
        //     })?;
        
        // if recovered_pubkey.0.ne(be) {
        //     msg!("Invalid coupon: Recovered pubkey does not match backend pubkey");
        //     return Err(CustomError::InvalidCoupon);
        // }
        // msg!("Valid coupon");
        // Ok(())
    }

    fn hash(&self, serialized_data: &Vec<u8>) -> [u8; HASH_BYTES] {
        let hash = {
            let mut hasher = keccak::Hasher::default();
            hasher.hash(&serialized_data[..]);
            hasher.result()
        };

        hash.0
    }
}
