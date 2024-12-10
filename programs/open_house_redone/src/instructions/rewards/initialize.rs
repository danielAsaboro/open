use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, Mint, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;
use crate::state::program_state::ProgramState;
use crate::state::user::User;
use crate::state::reward::RewardsTreasury;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RewardType {
    Vote,
    VoteReceived,
    Comment,
    Listing,
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = ProgramState::LEN, seeds = [b"program_state"], bump)]
    pub program_state: Account<'info, ProgramState>,

    #[account(init, payer = authority, mint::authority = authority, mint::decimals = 6)]
    pub rewards_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [b"rewards_treasury"],
        bump,
        token::mint = rewards_mint,
        token::authority = program_state
    )]
    pub rewards_treasury: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user, space = User::LEN, seeds = [b"user", user.key().as_ref()], bump)]
    pub user_state: Account<'info, User>,

    #[account(
        init,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub rewards_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeRewards<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = RewardsTreasury::LEN, seeds = [b"treasury"], bump)]
    pub treasury: Account<'info, RewardsTreasury>,

    pub system_program: Program<'info, System>,
}

// Implementation of initialize instructions
// pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
//     let program_state = &mut ctx.accounts.program_state;
//     program_state.authority = ctx.accounts.authority.key();
//     program_state.rewards_mint = ctx.accounts.rewards_mint.key();
//     program_state.rewards_treasury = ctx.accounts.rewards_treasury.key();
//     program_state.total_users = 0;
//     program_state.bump = ctx.bumps.program_state;
//     Ok(())
// }

pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    program_state.authority = ctx.accounts.authority.key();
    program_state.rewards_mint = ctx.accounts.rewards_mint.key();
    program_state.rewards_treasury = ctx.accounts.rewards_treasury.key();
    program_state.total_users = 0;

    // Get the bump from the PDA
    let (_, bump) = Pubkey::find_program_address(&[b"program_state"], ctx.program_id);
    program_state.bump = bump;

    Ok(())
}

pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    user_state.authority = ctx.accounts.user.key();
    user_state.tokens = 0;
    user_state.total_votes_cast = 0;
    user_state.total_votes_received = 0;
    user_state.comments_count = 0;
    user_state.listings_count = 0;
    user_state.last_reward_claim = Clock::get()?.unix_timestamp;
    user_state.bump = ctx.bumps.user_state;
    Ok(())
}

pub fn initialize_rewards(ctx: Context<InitializeRewards>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.authority = ctx.accounts.authority.key();
    treasury.total_tokens = 0;
    treasury.tokens_distributed = 0;
    treasury.bump = ctx.bumps.treasury;
    Ok(())
}
