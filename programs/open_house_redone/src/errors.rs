use anchor_lang::prelude::*;

#[error_code]
pub enum OpenHouseError {
    // Account Errors
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Unauthorized access attempt")]
    UnauthorizedAccess,

    // Token Errors
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Insufficient reward balance")]
    InsufficientRewardBalance,
    #[msg("Insufficient funds for operation")]
    InsufficientFunds,
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    #[msg("Arithmetic underflow occurred")]
    Underflow,

    // Content/Data Validation Errors
    #[msg("Review content exceeds maximum length")]
    ReviewTooLong,
    #[msg("Property ID exceeds maximum length")]
    PropertyIdTooLong,
    #[msg("Invalid property data provided")]
    InvalidPropertyData,
    #[msg("Invalid location data provided")]
    InvalidLocationData,

    // Voting Errors
    #[msg("Vote overflow occurred")]
    VoteOverflow,
    #[msg("Vote underflow occurred")]
    VoteUnderflow,
    #[msg("Duplicate vote detected")]
    DuplicateVote,
    #[msg("Invalid vote type")]
    InvalidVoteType,

    // Listing Errors
    #[msg("Listing is not active")]
    ListingNotActive,
    #[msg("Listing already exists")]
    ListingAlreadyExists,
    #[msg("Invalid listing status transition")]
    InvalidListingStatusTransition,

    // Comment Errors
    #[msg("Comment content is empty")]
    EmptyComment,
    #[msg("Comment exceeds maximum length")]
    CommentTooLong,
    #[msg("Invalid comment data")]
    InvalidCommentData,

    // Reward Errors
    #[msg("Reward distribution failed")]
    RewardDistributionFailed,
    #[msg("Invalid reward amount")]
    InvalidRewardAmount,
    #[msg("Daily reward already claimed")]
    DailyRewardAlreadyClaimed,

    // Program State Errors
    #[msg("Invalid program state")]
    InvalidProgramState,
    #[msg("Program state already initialized")]
    ProgramStateAlreadyInitialized,

    // System Errors
    #[msg("Invalid system clock")]
    InvalidSystemClock,
    #[msg("Operation temporarily unavailable")]
    OperationUnavailable,
}