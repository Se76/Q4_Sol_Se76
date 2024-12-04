use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked}};

use crate::{GiftConfig, GiftErrors};

#[derive(Accounts)]
pub struct AddToken<'info> {
    #[account(
        mut,
        seeds = [b"gift", creator.key().as_ref(), reciever.key().as_ref()],
        bump = gift_config.bump,
    )]
    pub gift_config: Account<'info, GiftConfig>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub reciever: SystemAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub creator_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = gift_config,
    )]
    pub config_ata: Account<'info, TokenAccount>,

    // #[account(init_if_needed)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info> AddToken<'info> {

    // ADD TOKEN v1

    // pub fn add_token(&mut self, amount_of_token: u64, decimals: u8) -> Result<()> {

        // self.gift_config.mints.push(self.mint.key());

        // let cpi_program = self.token_program.to_account_info();

        // let cpi_ctx = CpiContext::new(
        //     cpi_program,
        //     TransferChecked {
        //         from: self.creator_ata.to_account_info(),
        //         to: self.config_ata.to_account_info(),
        //         authority: self.creator.to_account_info(),
        //         mint: self.mint.to_account_info(),
        //     }
        // );

        // let cpi_invokation = transfer_checked(cpi_ctx, amount_of_token, decimals);
        // // self.gift_config.mints.push(self.asset.key());
        // msg!("added token___________________________________");
        // Ok(())

            // ADD TOKEN v2
            pub fn add_token(&mut self, amount_of_token: u64, decimals: u8) -> Result<()> {
                // Add the mint to the list of mints in gift_config
                require!(self.gift_config.mints.len() < 20, GiftErrors::VectorOfMintsIsTooLong);

                self.gift_config.mints.push(self.mint.key());
        
                if decimals == 0 {
                    msg!("Transferring NFT...");
                    let cpi_program = self.token_program.to_account_info();
                    let cpi_ctx = CpiContext::new(
                        cpi_program,
                        TransferChecked {
                            from: self.creator_ata.to_account_info(),
                            to: self.config_ata.to_account_info(),
                            authority: self.creator.to_account_info(),
                            mint: self.mint.to_account_info(),
                        },
                    );
        
                    transfer_checked(cpi_ctx, 1, 0)?;
        
                } else {
                    msg!("Transferring fungible token...");
                    let cpi_program = self.token_program.to_account_info();
                    let cpi_ctx = CpiContext::new(
                        cpi_program,
                        TransferChecked {
                            from: self.creator_ata.to_account_info(),
                            to: self.config_ata.to_account_info(),
                            authority: self.creator.to_account_info(),
                            mint: self.mint.to_account_info(),
                        },
                    );
        
                    transfer_checked(cpi_ctx, amount_of_token, decimals)?;
                }
        
                msg!("Token transfer completed.");
                Ok(())
            }
        }