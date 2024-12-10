use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Location {
    pub long: u64,
    pub lat: u64,
    pub address: String,
    pub nearest_bus_stop: String,
    pub landmark: Option<String>,
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
impl Comment {
    pub const LEN: usize =
        8 + // discriminator
        32 + // owner pubkey
        32 + // listing pubkey
        4 + // content string prefix
        8 + // vote_count
        1; // bump
}

// 2. Correct space calculation
impl Listing {
    pub const MAX_ADDRESS_LEN: usize = 100;
    pub const MAX_BUS_STOP_LEN: usize = 50;
    pub const MAX_LANDMARK_LEN: usize = 50;

    pub const LEN: usize =
        DISCRIMINATOR_SIZE +
        32 + // creator pubkey
        8 + // created timestamp
        8 + // updated timestamp
        1 + // status (enum variant)
        2 + // vote_count (i16)
        16 + // location (2 * u64)
        4 +
        Self::MAX_ADDRESS_LEN + // address string
        4 +
        Self::MAX_BUS_STOP_LEN + // bus_stop string
        4 +
        Self::MAX_LANDMARK_LEN; // optional landmark
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct ListingData {
    pub location: Location,
    pub creator: Pubkey,
    pub created: i64,
    pub updated: i64,
    pub status: ListingStatus,
}

//

pub struct ListingDataReal {
    pub creator: Pubkey,
    pub basic_info: BasicInfo,
    pub detail_info: DetailedInfo,
    pub created: i64,
    pub updated: i64,
    pub status: ListingStatus,
}

pub struct BasicInfo {
    pub title: String,
    pub status: ListingStatus,
    // #[(max_len = 200)]
    pub description: String,
    pub rent: u32,
    pub images: Vec<String>,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub parking_spaces: u8,
    pub area_sqft: u32,
}

pub struct DetailedInfo {
    pub location: Location,
}

#[derive(Debug)]
pub enum RentPeriod {
    Monthly(u32), // Amount per month
    Quarterly(u32), // Amount per quarter
    BiAnnually(u32), // Amount per Half-Year
    Annually(u32), // Amount per year
}
