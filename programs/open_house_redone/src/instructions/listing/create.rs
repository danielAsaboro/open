use anchor_lang::prelude::*;
use crate::state::listing::*;

#[derive(Accounts)]
#[instruction(location: Location)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = Listing::LEN,
        seeds = [
            b"listing",
            creator.key().as_ref(),
            &location.long.to_le_bytes(),
            &location.lat.to_le_bytes(),
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,
    pub system_program: Program<'info, System>,
}

pub fn create_listing(ctx: Context<CreateListing>, listing: Listing) -> Result<()> {
    let new_listing = &mut ctx.accounts.listing;
    new_listing.location = listing.location;
    new_listing.creator = ctx.accounts.creator.key();
    let clock: Clock = Clock::get().unwrap();
    new_listing.created = clock.unix_timestamp;
    new_listing.updated = clock.unix_timestamp;
    new_listing.status = ListingStatus::Active;
    Ok(())
}
