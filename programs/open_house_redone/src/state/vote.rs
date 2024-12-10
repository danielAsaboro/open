use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub owner: Pubkey,
    pub target: Pubkey,
    pub is_upvote: bool,
    pub bump: u8,
}

impl Vote {
    pub const LEN: usize = 
        8 + // discriminator
        32 + // owner pubkey
        32 + // target pubkey
        1 + // is_upvote
        1; // bump
}
