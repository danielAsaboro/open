// instructions/rewards/mint.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, MintTo };
use crate::state::program_state::ProgramState;

#[derive(Accounts)]
pub struct MintRewards<'info> {
    #[account(
        seeds = [b"program_state"],
        bump = program_state.bump,
        has_one = authority,
        has_one = rewards_mint
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub rewards_mint: Account<'info, Mint>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// Only program authority can mint tokens
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn mint_rewards(ctx: Context<MintRewards>, amount: u64) -> Result<()> {
    // Mint new tokens to the treasury
    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), MintTo {
            mint: ctx.accounts.rewards_mint.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        }),
        amount
    )?;

    Ok(())
}
