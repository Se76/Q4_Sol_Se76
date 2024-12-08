use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        MasterEditionAccount, 
        Metadata, 
        MetadataAccount
    }, 
    token::{
        approve, 
        Approve, 
        Mint, 
        Token, 
        TokenAccount
    }
};
use mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts};

use crate::state::{stake_account::StakeAccount, stake_config::StakeConfig, user_account::UserAccount};
use crate::errors::Error;
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>, // address of NFT
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()], // fixed seed for metaplex metadata
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), // we want to make sure, that the nft is part of the collection
        constraint = metadata.collection.as_ref().unwrap().verified == true, // we verify that this nft is part of mnetioned collection
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"], // set von Metaplex als Standart für MasterEditionAccount
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>, // verifies supply mint = 1, zero decimals and tests mint authority to master ediotion
    #[account(
        seeds = [b"config"],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        seeds = [b"stake", config_account.key().as_ref(), mint.key().as_ref()],
        bump,
        space = StakeAccount::INIT_SPACE,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}


impl <'info> Stake <'info> {
    pub fn stake(&mut self, bumps: StakeBumps) -> Result<()> {
        require!(self.user_account.amount_staked <= self.config_account.max_stake, Error::MaxStakeReached);


        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {                    // The user
            to: self.mint_ata.to_account_info(),                     // gives bassicly 
            delegate: self.stake_account.to_account_info(),          // Authority over min_ata to 
            authority: self.user.to_account_info(),                  // delegate account
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;                                 // The user gives Authority over mint_ata to delegate


        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config_account.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        ).invoke_signed(signer_seeds)?;

        self.stake_account.set_inner(
            StakeAccount{
                owner: self.user.key(),
                mint: self.mint.key(),
                staked_at: Clock::get()?.unix_timestamp,
                bump: bumps.stake_account,
            });

        self.user_account.amount_staked += 1;

        Ok(())
    }
}