use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked}};

use crate::{GiftConfig, GiftErrors}; // Gift erros were used in previous version

#[derive(Accounts)]
pub struct Unpack<'info> {
    #[account(
        mut,
        seeds = [b"gift", creator.key().as_ref(), reciever.key().as_ref()],
        bump = gift_config.bump,
        constraint = gift_config.creator == creator.key(),
        constraint = gift_config.reciever == reciever.key(),  
    )]
    pub gift_config: Account<'info, GiftConfig>,

    #[account(mut)]
    pub reciever: Signer<'info>,

    pub creator: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = reciever,
        associated_token::mint = mint,
        associated_token::authority = reciever,
    )]
    pub reciever_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = gift_config,
    )]
    pub config_ata: Account<'info, TokenAccount>,

    // #[account(
        // address = gift_config.mints[index as usize] 
        // @ GiftErrors::MintAddressNotInTheVector,
        
        // owner = gift_config.reciever.key() 
        // @ GiftErrors::WrongOwnerOfTheGift,
    // )]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl <'info> Unpack<'info> {
    pub fn unpack(&mut self) -> Result<()> {

        let seeds = &[
            &"gift".as_bytes(),
            &self.creator.key().to_bytes()[..],
            &self.reciever.key().to_bytes()[..],
            &[self.gift_config.bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            TransferChecked {
                from: self.config_ata.to_account_info(),
                to: self.reciever_ata.to_account_info(),
                authority: self.gift_config.to_account_info(),
                mint: self.mint.to_account_info(),
            },
            signer_seeds
        );

       
        transfer_checked(cpi_ctx, self.config_ata.amount, 6)?;


        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount
        {
            account: self.config_ata.to_account_info(),
            destination: self.reciever_ata.to_account_info(),
            authority: self.gift_config.to_account_info(),
        };

        let seeds = &[
            &"gift".as_bytes(),
            &self.creator.key().to_bytes()[..],
            &self.reciever.key().to_bytes()[..],
            &[self.gift_config.bump]];

        let signer_seeds = &[&seeds[..]];



        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds,
        );

        close_account(cpi_ctx)?;

        Ok(())
    }
}