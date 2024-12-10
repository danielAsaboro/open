use anchor_lang::prelude::*;
use crate::state::listing::*;
use crate::state::program_state::ProgramState;
use anchor_spl::token::{ self, Token, TokenAccount };
use crate::state::reward::RewardsTreasury;
use crate::state::user::User;

#[derive(Accounts)]
#[instruction(listing_data: ListingData)]
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
            &listing_data.location.long.to_le_bytes(),
            &listing_data.location.lat.to_le_bytes(),
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(seeds = [b"program_state"], bump)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub rewards_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: Account<'info, RewardsTreasury>,

    #[account(
        mut,
        seeds = [b"user", creator.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_listing(ctx: Context<CreateListing>, listing: ListingData) -> Result<()> {
    let new_listing = &mut ctx.accounts.listing;
    new_listing.location = listing.location;
    new_listing.creator = ctx.accounts.creator.key();
    let clock: Clock = Clock::get().unwrap();
    new_listing.created = clock.unix_timestamp;
    new_listing.updated = clock.unix_timestamp;
    new_listing.status = ListingStatus::Active;
    Ok(())
}
