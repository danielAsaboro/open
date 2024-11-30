use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy)]
pub struct Location {
    pub long: u64,
    pub lat: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq)]
pub enum ListingStatus {
    Active,
    Sold,
    Deleted,
}

#[account]
pub struct Listing {
    pub creator: Pubkey,
    pub location: Location,
    pub created: i64,
    pub updated: i64,
    pub status: ListingStatus,
    pub vote_count: i16,
}

#[account]
pub struct Comment {
    pub owner: Pubkey,
    pub listing: Pubkey,
    pub content: String,
    pub vote_count: i64,
    pub bump: u8,
}
impl Listing {
    pub const LEN: usize =
        DISCRIMINATOR_SIZE +
        16 + // location (2 * u64)
        32 + // creator pubkey
        8 + // timestamp
        8 +
        1 + // status (enum variant)
        8; // vote count
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy)]
pub struct ListingData {
    pub location: Location,
    pub creator: Pubkey,
    pub created: i64,
    pub updated: i64,
    pub status: ListingStatus,
}
