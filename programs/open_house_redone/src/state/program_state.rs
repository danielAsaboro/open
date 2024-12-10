use anchor_lang::prelude::*;

use crate::constants::{DISCRIMINATOR_SIZE, PUBKEY_SIZE, U64_SIZE};

#[account]
pub struct ProgramState {
    pub authority: Pubkey,
    pub rewards_mint: Pubkey,
    pub rewards_treasury: Pubkey,
    pub total_users: u64,
    pub bump: u8,
}

impl ProgramState {
    pub const LEN: usize =
        DISCRIMINATOR_SIZE +
        PUBKEY_SIZE + // authority
        PUBKEY_SIZE + // rewards_mint
        PUBKEY_SIZE + // rewards_treasury
        U64_SIZE + // total_users
        1; // bump
}
