use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;
pub mod state;
pub mod utils;

// pub mod constants; // Comentado porque el archivo no existe en este commit
// pub mod error;     // Comentado porque el archivo no existe en este commit

declare_id!("CDXwSVQSpQU5Dc8ivxbZAvd7DEJrxEy6bh8XS3u9mGQV");

#[program]
pub mod server {
    use super::*;

    // ADMIN

    pub fn init(ctx: Context<InitToken>, payload: InitPayload) -> Result<()> {
        // msg!("Program ID at runtime (from declare_id!): {}", ID); // Comentado
        instructions::admin::init::init(ctx, payload)
    }

    pub fn set_be(ctx: Context<ManageAdmin>, payload: [u8; 64]) -> Result<()> {
        instructions::admin::manage_admin::set_be(ctx, payload)
    }

    pub fn set_signer(ctx: Context<ManageAdmin>, payload: Pubkey) -> Result<()> {
        instructions::admin::manage_admin::set_signer(ctx, payload)
    }

    pub fn set_treasury_wallet(ctx: Context<SetTreasuryWallet>, new_wallet: Pubkey) -> Result<()> {
        instructions::admin::set_treasury_wallet::handler(ctx, new_wallet)
    }

    pub fn admin_reset_team_supply(ctx: Context<AdminResetTeamSupply>) -> Result<()> {
        instructions::admin::admin_reset_team_supply::handler(ctx)
    }

    // GITHUB

    pub fn add_repo(ctx: Context<AddRepo>, payload: AddRepoPayload) -> Result<()> {
        instructions::github::add_repo::add_repo(ctx, payload)
    }

    pub fn vote_repo<'info>(
        ctx: Context<'_, '_, 'info, 'info, VoteRepo<'info>>,
        payload: VoteRepoPayload,
    ) -> Result<()> {
        instructions::github::vote_repo::handler(ctx, payload)
    }

    pub fn subscribe(ctx: Context<Subscribe>, payload: SubscribePayload) -> Result<()> {
        instructions::github::subscribe::subscribe(ctx, payload)
    }

    pub fn finalize_project_proposal(ctx: Context<FinalizeProjectProposal>) -> Result<()> {
        instructions::github::finalize_project_proposal::handler(ctx)
    }

    // MINT

    pub fn claim_rewards(ctx: Context<ClaimRewards>, payload: ClaimRewardsPayload) -> Result<()> {
        instructions::mint::claim_rewards::claim_rewards(ctx, payload)
    }
    
    pub fn claim_team_tokens(ctx: Context<ClaimTeamTokens>) -> Result<()> {
        instructions::mint::claim_team_tokens::handler(ctx)
    }

    pub fn initialize_team_vesting(
        ctx: Context<InitializeTeamVesting>,
        payload: InitializeVestingPayload,
    ) -> Result<()> {
        instructions::mint::initialize_team_vesting::handler(ctx, payload)
    }

    pub fn mint_misc_tokens(ctx: Context<MintMiscTokens>, payload: MintMiscPayload) -> Result<()> {
        instructions::mint::mint_misc_tokens::handler(ctx, payload)
    }

    pub fn update_team_wallet(
        ctx: Context<UpdateTeamWallet>,
        payload: UpdateTeamWalletPayload,
    ) -> Result<()> {
        instructions::mint::update_team_wallet::handler(ctx, payload)
    }

    // REWARD

    pub fn create_grant(ctx: Context<CreateGrant>, payload: CreateGrantPayload) -> Result<()> {
        instructions::reward::create_grant::handler(ctx, payload)
    }

    pub fn mint_reward_from_grant(
        ctx: Context<MintRewardFromGrant>,
        payload: MintFromGrantPayload,
    ) -> Result<()> {
        instructions::reward::mint_from_grant::handler(ctx, payload)
    }

    pub fn update_grant_allocation(
        ctx: Context<UpdateGrantAllocation>,
        payload: UpdateGrantPayload,
    ) -> Result<()> {
        instructions::reward::update_grant::handler(ctx, payload)
    }

    // STAKING

    pub fn initialize_staking(ctx: Context<InitializeStaking>) -> Result<()> {
        instructions::staking::initialize::handler(ctx)
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        instructions::staking::stake::handler(ctx, amount)
    }

    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        instructions::staking::unstake::handler(ctx, amount)
    }

    pub fn claim_bytes(ctx: Context<ClaimBytes>) -> Result<()> {
        instructions::staking::claim_bytes::handler(ctx)
    }

    pub fn set_staking_rate(ctx: Context<SetStakingRate>, new_rate: u64) -> Result<()> {
        instructions::staking::set_staking_rate::handler(ctx, new_rate)
    }

    // BYTES

    // pub fn initialize_bytes_mint(ctx: Context<InitializeBytesMint>) -> Result<()> { // Comentada
    //     instructions::bytes::initialize_bytes_mint::handler(ctx)
    // }

    // REPUTATION

    pub fn initialize_reputation(ctx: Context<InitializeReputation>) -> Result<()> {
        instructions::reputation::initialize_reputation::handler(ctx)
    }
}
