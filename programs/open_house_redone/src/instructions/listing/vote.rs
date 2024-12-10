use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };

use crate::{
    errors::OpenHouseError,
    state::{
        listing::{ Listing, ListingStatus },
        program_state::ProgramState,
        reward::RewardsTreasury,
        user::User,
        vote::Vote,
    },
};

#[derive(Accounts)]
pub struct VoteOnListing<'info> {
    #[account(
        mut,
        constraint = listing.status == ListingStatus::Active @ OpenHouseError::ListingNotActive
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        init_if_needed,
        seeds = [b"listing_vote", listing.key().as_ref(), voter.key().as_ref()],
        space = Vote::LEN,
        bump,
        payer = voter
    )]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(seeds = [b"program_state"], bump)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub rewards_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub voter_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub listing_creator_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: Account<'info, RewardsTreasury>,

    #[account(
        mut,
        seeds = [b"user", voter.key().as_ref()],
        bump
    )]
    pub voter_user: Account<'info, User>,

    #[account(
        mut,
        seeds = [b"user", listing.creator.key().as_ref()],
        bump
    )]
    pub listing_creator_user: Account<'info, User>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn up_vote_listing(ctx: Context<VoteOnListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let vote = &mut ctx.accounts.vote;

    // Check for vote overflow
    require!(listing.vote_count < i16::MAX, OpenHouseError::VoteOverflow);

    // Validate existing vote
    if vote.owner == ctx.accounts.voter.key() {
        require!(!vote.is_upvote, OpenHouseError::DuplicateVote);
        listing.vote_count = listing.vote_count.checked_add(2).ok_or(OpenHouseError::VoteOverflow)?;
        vote.is_upvote = true;
        return Ok(());
    }

    // Process new vote
    vote.owner = ctx.accounts.voter.key();
    vote.target = listing.key();
    vote.is_upvote = true;
    vote.bump = ctx.bumps.vote;

    listing.vote_count = listing.vote_count.checked_add(1).ok_or(OpenHouseError::VoteOverflow)?;

    Ok(())
}

pub fn down_vote_listing(ctx: Context<VoteOnListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let vote = &mut ctx.accounts.vote;

    // If vote already exists, check if we need to change it
    if vote.owner == ctx.accounts.voter.key() {
        if !vote.is_upvote {
            // Already downvoted
            return Ok(());
        } else {
            // Change upvote to downvote
            // Remove upvote and add downvote
            listing.vote_count -= 2;
            vote.is_upvote = false;
            return Ok(());
        }
    }

    // New vote
    vote.owner = ctx.accounts.voter.key();
    vote.target = listing.key();
    vote.is_upvote = false;
    vote.bump = ctx.bumps.vote;

    listing.vote_count -= 1;

    Ok(())
}
