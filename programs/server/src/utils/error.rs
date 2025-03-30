use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User has already voted")]
    VotedAlready,

    #[msg("Commits already claimed")]
    ClaimedAlready,

    #[msg("This coupon is invalid")]
    InvalidCoupon,

    #[msg("Repo needs to be approved")]
    UnapprovedRepo,

    #[msg("Admin only")]
    AdminOnly,

    #[msg("Max supply exceeded")]
    MaxSupplyExceeded,

    #[msg("Invalid user")]
    InvalidUser,

    #[msg("Unauthorized action.")]
    Unauthorized,

    #[msg("Grant allocation exceeded.")]
    GrantExceeded,

    #[msg("Invalid grant update.")]
    InvalidGrantUpdate,

    #[msg("Rewards supply exceeded.")]
    RewardsSupplyExceeded,

    #[msg("Missing bump seed.")]
    MissingBump,

    #[msg("Arithmetic overflow.")]
    Overflow,

    #[msg("Total percentage is invalid.")]
    InvalidPercentageTotal,

    #[msg("Nothing available to claim.")]
    NothingToClaim,
}
