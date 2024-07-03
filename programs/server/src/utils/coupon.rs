use crate::borsh;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use solana_program::{
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
    const ADMIN_PUB_KEY: &'static str = "52a51c1bef8056119d5f114af2d71a2e978a9b260e1a156c2af1a1643291b0e90c38da6d1bef18fc80588dddfc5344638954482c7de6613a0c5eef6ec2e36ee3";

    pub fn verify(&self, serialized_data: &Vec<u8>) -> Result<String, CustomError> {
        msg!("Validating coupon");
        let hash = self.hash(serialized_data);

        let signature = &hex::decode(&self.signature).unwrap();

        let recovered_pubkey: Secp256k1Pubkey =
            secp256k1_recover(&hash, self.recovery_id, signature).unwrap();
        let recovered_pubkey_hex = hex::encode(recovered_pubkey.0);

        if recovered_pubkey_hex.ne(Coupon::ADMIN_PUB_KEY) {
            msg!("Invalid coupon");
            return Err(CustomError::InvalidCoupon.into());
        }
        msg!("Valid coupon");
        Ok(recovered_pubkey_hex)
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
