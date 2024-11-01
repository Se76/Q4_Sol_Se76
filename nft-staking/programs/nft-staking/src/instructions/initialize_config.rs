use anchor_lang::{prelude::*, Bumps};
use anchor_spl::{associated_token, mint, token::{Mint, Token}, token_interface::TokenInterface};

use crate::state::stake_config::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig <'info> {
    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump,
    )]
    pub config_account: Account<'info, StakeConfig>,


    #[account(
        init,
        payer = admin,
        seeds = [b"rewards".as_ref(), config_account.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config_account,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl <'info> InitializeConfig <'info> {
    pub fn initialize_config(&mut self, points_per_stake: u8, max_stake: u8, freeze_period: u32, bumps: &InitializeBumps) -> Result<()> {
        self.config_account.set_inner(StakeConfig{
            points_per_stake,
            rewards_bump: bumps.rewards_mint,
            max_stake,
            freeze_period,
            bump: bumps.config_account,

        });

        Ok(())
    }

}