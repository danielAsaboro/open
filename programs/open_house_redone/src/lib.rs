use anchor_lang::prelude::*;

declare_id!("J4B6mY3BfhpXmthVpdY6R315RTKxQnzitjYafwQZ8he");

#[program]
pub mod open_house_redone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
