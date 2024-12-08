mod state;
mod instructions;
mod errors;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("HM4AkpT6CHXaKVJS3UUT3oDiwD9ctx3vSo8GPNfPiMgB");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
}

