use anchor_lang::prelude::*;
use crate::state::listing::*;

#[account]
pub struct Vote {
    pub owner: Pubkey,
    // Can be either listing or comment Pubkey
    pub target: Pubkey,
    pub is_upvote: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct VoteOnListing<'info> {
    #[account(mut)]
    pub listing: Account<'info, Listing>,

    #[account(
        init_if_needed,
        seeds = [b"listing_vote", listing.key().as_ref(), voter.key().as_ref()],
        space = 8 + 32 + 32 + 1 + 1,
        bump,
        payer = voter
    )]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnComment<'info> {
    #[account(mut)]
    pub comment: Account<'info, Comment>,

    #[account(
        init_if_needed,
        seeds = [b"comment_vote", comment.key().as_ref(), voter.key().as_ref()],
        space = 8 + 32 + 32 + 1 + 1,
        bump,
        payer = voter
    )]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn up_vote_listing(ctx: Context<VoteOnListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let vote = &mut ctx.accounts.vote;

    // If vote already exists, check if we need to change it
    if vote.owner == ctx.accounts.voter.key() {
        if vote.is_upvote {
            return Ok(());
        } else {
            // Change downvote to upvote
            // Remove downvote and add upvote
            listing.vote_count += 2;
            vote.is_upvote = true;
            return Ok(());
        }
    }

    // New vote
    vote.owner = ctx.accounts.voter.key();
    vote.target = listing.key();
    vote.is_upvote = true;
    vote.bump = ctx.bumps.vote;

    listing.vote_count += 1;

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

// Similar implementation for comment voting
pub fn up_vote_comment(ctx: Context<VoteOnComment>) -> Result<()> {
    let comment = &mut ctx.accounts.comment;
    let vote = &mut ctx.accounts.vote;

    if vote.owner == ctx.accounts.voter.key() {
        if vote.is_upvote {
            return Ok(());
        } else {
            comment.vote_count += 2;
            vote.is_upvote = true;
            return Ok(());
        }
    }

    vote.owner = ctx.accounts.voter.key();
    vote.target = comment.key();
    vote.is_upvote = true;
    vote.bump = ctx.bumps.vote;

    comment.vote_count += 1;

    Ok(())
}

pub fn down_vote_comment(ctx: Context<VoteOnComment>) -> Result<()> {
    let comment = &mut ctx.accounts.comment;
    let vote = &mut ctx.accounts.vote;

    if vote.owner == ctx.accounts.voter.key() {
        if !vote.is_upvote {
            return Ok(());
        } else {
            comment.vote_count -= 2;
            vote.is_upvote = false;
            return Ok(());
        }
    }

    vote.owner = ctx.accounts.voter.key();
    vote.target = comment.key();
    vote.is_upvote = false;
    vote.bump = ctx.bumps.vote;

    comment.vote_count -= 1;

    Ok(())
}
