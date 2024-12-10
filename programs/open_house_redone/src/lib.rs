use anchor_lang::prelude::*;

declare_id!("J4B6mY3BfhpXmthVpdY6R315RTKxQnzitjYafwQZ8he");

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use crate::instructions::{
    comment::create::*,
    comment::vote::*,
    listing::create::*,
    listing::update::*,
    listing::vote::*,
    rewards::distribute::*,
    rewards::initialize::*,
    rewards::mint::*,
};

use crate::state::listing::{ ListingData, ListingStatus, Location };

#[program]
pub mod open_house_redone {
    use super::*;

    pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
        instructions::rewards::initialize::initialize_program(ctx)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::rewards::initialize::initialize_user(ctx)
    }

    pub fn initialize_rewards(ctx: Context<InitializeRewards>) -> Result<()> {
        instructions::rewards::initialize::initialize_rewards(ctx)
    }

    pub fn create_listing(ctx: Context<CreateListing>, listing_data: ListingData) -> Result<()> {
        instructions::listing::create::create_listing(ctx, listing_data)
    }

    pub fn update_listing(
        ctx: Context<UpdateListing>,
        new_location: Option<Location>,
        new_status: Option<ListingStatus>
    ) -> Result<()> {
        instructions::listing::update::update_listing(ctx, new_location, new_status)
    }

    pub fn up_vote_listing(ctx: Context<VoteOnListing>) -> Result<()> {
        instructions::listing::vote::up_vote_listing(ctx)
    }

    pub fn down_vote_listing(ctx: Context<VoteOnListing>) -> Result<()> {
        instructions::listing::vote::down_vote_listing(ctx)
    }

    pub fn create_comment(ctx: Context<CreateComment>, content: String) -> Result<()> {
        instructions::comment::create::create_comment(ctx, content)
    }

    pub fn up_vote_comment(ctx: Context<VoteOnComment>) -> Result<()> {
        instructions::comment::vote::up_vote_comment(ctx)
    }

    pub fn down_vote_comment(ctx: Context<VoteOnComment>) -> Result<()> {
        instructions::comment::vote::down_vote_comment(ctx)
    }

    pub fn mint_rewards(ctx: Context<MintRewards>, amount: u64) -> Result<()> {
        instructions::rewards::mint::mint_rewards(ctx, amount)
    }

    pub fn distribute_reward(
        ctx: Context<DistributeReward>,
        amount: u64,
        reward_type: RewardType
    ) -> Result<()> {
        instructions::rewards::distribute::distribute_reward(ctx, amount, reward_type)
    }
}
