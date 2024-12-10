// state/rewards.rs
use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR_SIZE, PUBKEY_SIZE, U64_SIZE};

#[account]
pub struct RewardsTreasury {
    pub authority: Pubkey,
    pub total_tokens: u64,
    pub tokens_distributed: u64,
    pub bump: u8,
}

impl RewardsTreasury {
    pub const LEN: usize =
        DISCRIMINATOR_SIZE +
        PUBKEY_SIZE + // authority
        U64_SIZE + // total_tokens
        U64_SIZE + // tokens_distributed
        1; // bump
}
