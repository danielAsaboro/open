// instructions/rewards/distribute.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount };

use crate::{
    errors::OpenHouseError,
    state::{ program_state::ProgramState, reward::RewardsTreasury },
};

use super::{ initialize::RewardType, User };

#[derive(Accounts)]
pub struct DistributeReward<'info> {
    #[account(seeds = [b"program_state"], bump = program_state.bump)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub rewards_treasury: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"user", recipient.key().as_ref()],
        bump = user.bump,
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub treasury: Account<'info, RewardsTreasury>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is the user receiving the reward
    pub recipient: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn distribute_reward(
    ctx: Context<DistributeReward>,
    amount: u64,
    reward_type: RewardType
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let user = &mut ctx.accounts.user;

    // Check if treasury has enough tokens
    require!(treasury.total_tokens >= amount, OpenHouseError::InsufficientFunds);

    // Update treasury balances
    treasury.total_tokens = treasury.total_tokens
        .checked_sub(amount)
        .ok_or(OpenHouseError::Overflow)?;

    treasury.tokens_distributed = treasury.tokens_distributed
        .checked_add(amount)
        .ok_or(OpenHouseError::Overflow)?;

    // Update user token balance
    user.tokens = user.tokens.checked_add(amount).ok_or(OpenHouseError::Overflow)?;

    // Update user stats based on reward type
    match reward_type {
        RewardType::Vote => {
            user.total_votes_cast = user.total_votes_cast
                .checked_add(1)
                .ok_or(OpenHouseError::Overflow)?;
        }
        RewardType::VoteReceived => {
            user.total_votes_received = user.total_votes_received
                .checked_add(1)
                .ok_or(OpenHouseError::Overflow)?;
        }
        RewardType::Comment => {
            user.comments_count = user.comments_count
                .checked_add(1)
                .ok_or(OpenHouseError::Overflow)?;
        }
        RewardType::Listing => {
            user.listings_count = user.listings_count
                .checked_add(1)
                .ok_or(OpenHouseError::Overflow)?;
        }
    }

    // Transfer tokens using CPI
    let seeds = &[b"program_state".as_ref(), &[ctx.accounts.program_state.bump]];
    let signer = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.rewards_treasury.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            signer
        ),
        amount
    )?;

    Ok(())
}
