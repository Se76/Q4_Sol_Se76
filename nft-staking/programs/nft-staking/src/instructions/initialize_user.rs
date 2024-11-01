use anchor_lang::prelude::*;

use crate::state::user_account::UserAccount;
use crate::state::user_account::*;



#[derive(Accounts)]
pub struct Initialize <'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE,
        seeds = [b"UserAccount".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>
}

impl <'info> Initialize <'info>{
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.user_account.amount_staked += 1;
        self.user_account.set_inner(UserAccount{
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });

        Ok(())
    }
}