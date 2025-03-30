use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("7dPueMoFZHG9Ae1GFX2FdVcZTjqFsvV6EhsUvW8Hhg8o");

#[program]
pub mod server {

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

    pub fn init(ctx: Context<InitToken>, payload: InitPayload) -> Result<()> {
        instructions::init::init(ctx, payload)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, payload: ClaimRewardsPayload) -> Result<()> {
        instructions::claim_rewards::claim_rewards(ctx, payload)
    }

    pub fn subscribe(ctx: Context<Subscribe>, payload: SubscribePayload) -> Result<()> {
        instructions::subscribe::subscribe(ctx, payload)
    }

    pub fn propose_project(ctx: Context<ProposeProject>, payload: ProposeProjectPayload) -> Result<()> {
        instructions::github::propose_project::handler(ctx, payload)
    }

    // ADMIN

    pub fn set_be(ctx: Context<ManageAdmin>, payload: [u8; 64]) -> Result<()> {
        instructions::set_be(ctx, payload)
    }

    pub fn set_signer(ctx: Context<ManageAdmin>, payload: Pubkey) -> Result<()> {
        instructions::set_signer(ctx, payload)
    }

    pub fn claim_team_tokens(ctx: Context<ClaimTeamTokens>) -> Result<()> {
        instructions::claim_team_tokens::handler(ctx)
    }

    pub fn create_grant(ctx: Context<CreateGrant>, payload: CreateGrantPayload, repo_id: Pubkey) -> Result<()> {
        instructions::create_grant::handler(ctx, payload)
    }
    
    pub fn update_grant_allocation(ctx: Context<UpdateGrantAllocation>, payload: UpdateGrantPayload) -> Result<()> {
        instructions::update_grant::handler(ctx, payload)
    }
    
    pub fn mint_reward_from_grant(ctx: Context<MintRewardFromGrant>, payload: MintFromGrantPayload) -> Result<()> {
        instructions::mint_from_grant::handler(ctx, payload)
    }

    pub fn initialize_team_vesting(ctx: Context<InitializeTeamVesting>, payload: InitializeVestingPayload) -> Result<()> {
        instructions::initialize_team_vesting::handler(ctx, payload)
    }

    pub fn update_team_wallet(ctx: Context<UpdateTeamWallet>, payload: UpdateTeamWalletPayload) -> Result<()> {
        instructions::update_team_wallet::handler(ctx, payload)
    }

    // STAKING

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, amount)
    }
    
    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        instructions::unstake::handler(ctx, amount)
    }

    pub fn claim_bytes(ctx: Context<ClaimBytes>) -> Result<()> {
        instructions::staking::claim_bytes::handler(ctx)
    }

    pub fn initialize_bytes_mint(ctx: Context<InitializeBytesMint>) -> Result<()> {
        instructions::bytes::initialize_bytes_mint::handler(ctx)
    }
}
