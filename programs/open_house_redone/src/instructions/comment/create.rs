use anchor_lang::prelude::*;
use crate::state::listing::*;

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
        space = 8 + // discriminator
        32 + // owner pubkey
        32 + // listing pubkey
        4 +
        content.len() + // string (4 bytes for length prefix + content)
        8 + // vote_count (i64)
        1, // bump
        seeds = [b"comment", listing.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub comment: Account<'info, Comment>,

    pub system_program: Program<'info, System>,
}

pub fn create_comment(ctx: Context<CreateComment>, content: String) -> Result<()> {
    let comment = &mut ctx.accounts.comment;

    comment.owner = ctx.accounts.owner.key();
    comment.listing = ctx.accounts.listing.key();
    comment.content = content;
    comment.vote_count = 0;
    comment.bump = ctx.bumps.comment;

    Ok(())
}
