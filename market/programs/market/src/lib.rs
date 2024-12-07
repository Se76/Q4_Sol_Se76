use anchor_lang::prelude::*;

declare_id!("76ofariHewGmFaaeP1NtuxoEMyyNj1wYxapieW22Cnff");

#[program]
pub mod market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
