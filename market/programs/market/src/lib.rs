mod instructions;
mod state;
mod errors;

use anchor_lang::prelude::*;
use crate::instructions::*;



declare_id!("5veWFCgrb8zE3vPHogD3532wvZxvqUqqa2iBw31PjF3T");

#[program]
pub mod market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;    
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_nft()?;
        ctx.accounts.send_sol()?;
        ctx.accounts.send_rewards()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
}

