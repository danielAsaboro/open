use anchor_lang::prelude::*;

use crate::{errors::OpenHouseError, state::listing::{ Listing, ListingStatus, Location }};

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        has_one = creator ,
        constraint = listing.status == ListingStatus::Active,
        seeds = [
            b"listing",
            creator.key().as_ref(),
            &listing.location.long.to_le_bytes(),
            &listing.location.lat.to_le_bytes(),
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,
}

pub fn update_listing(
    ctx: Context<UpdateListing>,
    new_location: Option<Location>,
    new_status: Option<ListingStatus>
) -> Result<()> {
    let listing = &mut ctx.accounts.listing;

    if let Some(location) = new_location {
        listing.location = location;
    }

    if let Some(status) = new_status {
        match (listing.status, status) {
            (ListingStatus::Active, ListingStatus::Sold) => (),
            (ListingStatus::Active, ListingStatus::Deleted) => (),
            (_, _) => {
                return Err(OpenHouseError::InvalidListingStatusTransition.into());
            }
        }
        listing.status = status;
    }

    listing.updated = Clock::get()?.unix_timestamp;

    Ok(())
}
