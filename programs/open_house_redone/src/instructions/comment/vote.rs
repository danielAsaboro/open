use anchor_lang::prelude::*;

use crate::state::{listing::Comment, vote::Vote};

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
