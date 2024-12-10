// constants.rs

pub const DISCRIMINATOR_SIZE: usize = 8;
pub const PUBKEY_SIZE: usize = 32;
pub const STRING_PREFIX_SIZE: usize = 4;
pub const BOOL_SIZE: usize = 1;
pub const U64_SIZE: usize = 8;
pub const I64_SIZE: usize = 8;
pub const VEC_PREFIX_SIZE: usize = 4;

// Program-specific constants
pub const MAX_PROPERTY_ID_LENGTH: usize = 50;
pub const MAX_REVIEW_CONTENT_LENGTH: usize = 200;

// location reveal fee
pub const LOCATION_REVEAL_FEE: u64 = 10000;

// constants/rewards.rs
pub const VOTE_REWARD: u64 = 1; // Reward for casting a vote
pub const VOTE_RECEIVED_REWARD: u64 = 2; // Reward for receiving a vote
pub const COMMENT_REWARD: u64 = 5; // Reward for making a comment
pub const LISTING_REWARD: u64 = 10; // Reward for creating a listing
pub const DAILY_PARTICIPATION_REWARD: u64 = 20; // Daily reward for active users
