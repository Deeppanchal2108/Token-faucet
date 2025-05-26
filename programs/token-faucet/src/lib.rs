use anchor_lang::prelude::*;

declare_id!("EXCXR7BgBQjnsJGz1BbNTm5uK1wTJFQL3fCm9fVUwJVB");

#[program]
pub mod token_faucet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
