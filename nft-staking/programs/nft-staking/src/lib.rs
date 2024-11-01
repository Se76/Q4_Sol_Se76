mod state;
mod instructions;
mod errors;

use anchor_lang::prelude::*;
use crate::state::*;
use crate::instructions::*;
use crate::errors;

declare_id!("HM4AkpT6CHXaKVJS3UUT3oDiwD9ctx3vSo8GPNfPiMgB");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
