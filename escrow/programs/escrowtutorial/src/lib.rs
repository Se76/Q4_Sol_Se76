pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2NXYwU37VaoP5tKWfu9sCDTEvSfBiGj82jL64jjiYrKq");

#[program]
pub mod escrowtutorial {
    use crate::make::Make;

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, recieve: u64) -> Result<()> {
        ctx.accounts.initialize_escrow(seed, recieve, &ctx.bumps)?;
        ctx.accounts.tranfer_mint_a_to_vault(deposit);
        Ok(())
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.transfer_and_close_vault()?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>, ) -> Result<()>{
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
}
