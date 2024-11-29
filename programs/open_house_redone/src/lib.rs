use anchor_lang::prelude::*;
use crate::state::listing::{ Listing, ListingStatus, Location };
use crate::instructions::listing::{ create::*, update::* };

pub mod state;
pub mod instructions;
pub mod constants;

declare_id!("J4B6mY3BfhpXmthVpdY6R315RTKxQnzitjYafwQZ8he");

#[program]
pub mod open_house_redone {
    use super::*;

    pub fn create_listing(ctx: Context<CreateListing>, listing: Listing) -> Result<()> {
        instructions::listing::create::create_listing(ctx, listing)
    }

    pub fn update_listing(
        ctx: Context<UpdateListing>,
        new_location: Option<Location>,
        new_status: Option<ListingStatus>
    ) -> Result<()> {
        instructions::listing::update::update_listing(ctx, new_location, new_status)
    }
}

// property_id: String,
// verify_access
// handle location reveal
// submit_review(
// distribute_rewards(

// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// pub enum VoteType {
//     Upvote,
//     Downvote,
// }

// #[account]
// try to implement initspace later, feels cleaner
// pub struct Review {
//     pub content: String,
//     pub votes: i64,
//     pub property_id: String,
//     pub renter: Pubkey,
//     pub voted_users: Vec<Pubkey>,
// }

// Validate property ID length
//   require!(
//     property_id.len() <= MAX_PROPERTY_ID_LENGTH,
//     CustomError::PropertyIdTooLong);
// do we need a property id?