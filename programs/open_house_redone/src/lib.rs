use anchor_lang::prelude::*;
use crate::state::listing::{ ListingData, ListingStatus, Location };
use crate::instructions::listing::{ create::*, update::*, vote::* };
use crate::instructions::comment::create::*;

pub mod state;
pub mod instructions;
pub mod constants;

declare_id!("J4B6mY3BfhpXmthVpdY6R315RTKxQnzitjYafwQZ8he");

#[program]
pub mod open_house_redone {
    use super::*;

    pub fn create_listing(ctx: Context<CreateListing>, listing: ListingData) -> Result<()> {
        instructions::listing::create::create_listing(ctx, listing)
    }

    pub fn update_listing(
        ctx: Context<UpdateListing>,
        new_location: Option<Location>,
        new_status: Option<ListingStatus>
    ) -> Result<()> {
        instructions::listing::update::update_listing(ctx, new_location, new_status)
    }

    pub fn vote_on_listing(ctx: Context<VoteOnListing>, is_up_vote: bool) -> Result<()> {
        if is_up_vote {
            return instructions::listing::vote::up_vote_listing(ctx);
        } else {
            return instructions::listing::vote::down_vote_listing(ctx);
        }
    }

    pub fn vote_on_listing_comment(ctx: Context<VoteOnComment>, is_up_vote: bool) -> Result<()> {
        if is_up_vote {
            return instructions::listing::vote::up_vote_comment(ctx);
        } else {
            return instructions::listing::vote::down_vote_comment(ctx);
        }
    }

    pub fn create_comment(ctx: Context<CreateComment>, content: String) -> Result<()> {
        instructions::comment::create::create_comment(ctx, content)
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
