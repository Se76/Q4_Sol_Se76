use anchor_lang::prelude::*;
use anchor_spl::{token_interface::TokenInterface, associated_token::AssociatedToken, token::{Token, TokenAccount, Mint, TransferChecked, transfer_checked, CloseAccount, close_account}};
use crate::{instructions::make::*, Escrow, escrow};

#[derive(Accounts)]
pub struct Refund <'info>{
    #[account(mut)]
    pub maker: Signer<'info>, // escrow
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,  
    pub token_program: Program<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl <'info> Refund <'info>{
    pub fn transfer_and_close_vault(&mut self) -> Result<()>{

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let cpi_program = self.token_program.to_account_info();

        let accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, accounts, &signer_seeds);

        // let tx = transfer_checked(token_program_id, source_pubkey, mint_pubkey, destination_pubkey, authority_pubkey, signer_pubkeys, amount, decimals)

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;



        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }

    
}