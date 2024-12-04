use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

pub use crate::instructions::*;
pub use crate::state::*;
use crate::errors::*;

declare_id!("5CSHVvjWKynHo2kkCFRTVJhJ1fg1k2FR9RSDzQa38VMb");

#[program]
pub mod final_capstone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greetings: String) -> Result<()> {
        ctx.accounts.initialize(greetings, &ctx.bumps)?;
        Ok(())
    }

    pub fn add_token(ctx: Context<AddToken>, amount_of_token: u64, decimals: u8) -> Result<()> {
        ctx.accounts.add_token(amount_of_token, decimals)?;
        Ok(())
    }

    pub fn create_collection(ctx: Context<CollectionAccounts>) -> Result<()> {
        ctx.accounts.create_collection()?; 
        Ok(())
    }

    pub fn mint_and_send_nft(ctx: Context<MintAndSendNft>) -> Result<()> {
        ctx.accounts.mint_and_send_nft()?;
        Ok(())
    }

    pub fn burn(ctx: Context<Burn>) -> Result<()> {
        ctx.accounts.burn()?;
        Ok(())
    }

    pub fn unpack(ctx: Context<Unpack>) -> Result<()> {
        ctx.accounts.unpack()?;
        Ok(())
    }
}
