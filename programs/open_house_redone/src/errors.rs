// custom error definitions
#[error_code]
pub enum CustomError {
    #[msg("Review content is too long")]
    ReviewTooLong,
    #[msg("Property ID is too long")]
    PropertyIdTooLong,
    #[msg("Vote overflow occurred")]
    VoteOverflow,
    #[msg("Vote underflow occurred")]
    VoteUnderflow,
    #[msg("Duplicate vote detected")]
    DuplicateVote,
}

/// Error codes for custom errors
#[error_code]
pub enum OpenHouseError {
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Overflow occurred during token operations")]
    Overflow,
    #[msg("Unauthorized Access")]
    UnauthorizedAccess,
}
