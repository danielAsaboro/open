// state/user.rs
use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct User {
    pub authority: Pubkey, // User's wallet
    pub tokens: u64, // Current token balance
    pub total_votes_cast: u64, // Total number of votes given
    pub total_votes_received: u64, // Total votes received on content
    pub comments_count: u64, // Total comments made
    pub listings_count: u64, // Total listings created
    pub last_reward_claim: i64, // Timestamp of last reward claim
    pub bump: u8,
}

impl User {
    pub const LEN: usize =
        DISCRIMINATOR_SIZE +
        PUBKEY_SIZE + // authority
        U64_SIZE + // tokens
        U64_SIZE + // total_votes_cast
        U64_SIZE + // total_votes_received
        U64_SIZE + // comments_count
        U64_SIZE + // listings_count
        I64_SIZE + // last_reward_claim
        1; // bump
}
