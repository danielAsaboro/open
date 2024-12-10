use anchor_lang::prelude::*;
use crate::state::listing::Listing;

#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = Listing::LEN,
        seeds = [b"profile", owner.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug)]
pub struct UserProfile {
    name: String,
    email: String,
    location: String,
    contributions: u32,
    tokens_earned: u32,
    created: i64,
    updated: i64,
}

impl UserProfile {
    fn new(name: String, email: String, location: String) -> Self {
        UserProfile {
            name,
            email,
            location,
            tokens_earned: 0,
            contributions: 0,
            created: Clock::get().unwrap().unix_timestamp,
            updated: Clock::get().unwrap().unix_timestamp,
        }
    }
}

pub fn create_user_profile(ctx: Context<CreateUserProfile>) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    user_profile.name = String::new(); // Will be set later
    user_profile.email = String::new(); // Will be set later
    user_profile.location = String::new(); // Will be set later
    user_profile.contributions = 0;
    user_profile.tokens_earned = 0;
    user_profile.created = clock.unix_timestamp;
    user_profile.updated = clock.unix_timestamp;

    Ok(())
}
