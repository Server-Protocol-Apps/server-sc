use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("8Jy1eMYr3fjGHBAbW5ebT5tTssXtB4BQpWRzZRHk4HMg");

#[program]
pub mod smart_contract {

    use super::*;

    pub fn add_repo(ctx: Context<AddRepo>, payload: AddRepoPayload) -> Result<()> {
        instructions::add_repo::add_repo(ctx, payload)
    }

    pub fn vote_repo(ctx: Context<VoteRepo>, payload: VoteRepoPayload) -> Result<()> {
        instructions::vote_repo::vote_repo(ctx, payload)
    }

    pub fn verify_coupon(ctx: Context<VerifyCoupon>, payload: VerifyCouponPayload) -> Result<()> {
        instructions::verify_coupon::verify_coupon(ctx, payload)
    }

    pub fn init_token(ctx: Context<InitToken>) -> Result<()> {
        instructions::init_token::init_token(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, payload: ClaimRewardsPayload) -> Result<()> {
        instructions::claim_rewards::claim_rewards(ctx, payload)
    }

    pub fn subscribe(ctx: Context<Subscribe>, payload: SubscribePayload) -> Result<()> {
        instructions::subscribe::subscribe(ctx, payload)
    }
}
