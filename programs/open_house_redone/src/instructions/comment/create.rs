use anchor_lang::prelude::*;
use crate::state::listing::*;
use anchor_spl::token::{ self, Token, TokenAccount };
use crate::errors::OpenHouseError;
use crate::constants::MAX_REVIEW_CONTENT_LENGTH;
use crate::state::program_state::ProgramState;
use crate::state::reward::RewardsTreasury;
use crate::state::user::User;

#[derive(Accounts)]
#[instruction(content: String)]
pub struct CreateComment<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub listing: Account<'info, Listing>,

    #[account(
        init,
        payer = owner,
        space = Comment::LEN + content.len(),
        seeds = [b"comment", listing.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub comment: Account<'info, Comment>,

    #[account(seeds = [b"program_state"], bump)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub rewards_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury: Account<'info, RewardsTreasury>,

    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_comment(ctx: Context<CreateComment>, content: String) -> Result<()> {
    require!(!content.is_empty(), OpenHouseError::EmptyComment);
    require!(content.len() <= MAX_REVIEW_CONTENT_LENGTH, OpenHouseError::CommentTooLong);
    require!(
        ctx.accounts.listing.status == ListingStatus::Active,
        OpenHouseError::ListingNotActive
    );

    let comment = &mut ctx.accounts.comment;
    comment.owner = ctx.accounts.owner.key();
    comment.listing = ctx.accounts.listing.key();
    comment.content = content;
    comment.vote_count = 0;
    comment.bump = ctx.bumps.comment;

    Ok(())
}
