use anchor_lang::prelude::*;
use crate::{GiftConfig, GiftErrors};
use mpl_core::{
    accounts::BaseCollectionV1, instructions::BurnV1CpiBuilder, ID as MPL_CORE_PROGRAM_ID 
};


#[derive(Accounts)]
pub struct Burn<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>, // creator
    #[account(
        mut,
        constraint = collection.update_authority == payer.key(),
    )]
    pub collection: Account<'info, BaseCollectionV1>,
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub reciever: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"gift", payer.key().as_ref(), reciever.key().as_ref()],
        bump = gift_config.bump,
        constraint = gift_config.gift_nft_mint == asset.key(),
    )]
    pub gift_config: Account<'info, GiftConfig>,


}

impl <'info> Burn<'info> {
    pub fn burn(&mut self) -> Result<()> {
        require!(self.gift_config.gift_nft_mint == self.asset.key(), GiftErrors::NotValidGiftNFT);
        require!(self.gift_config.reciever == self.reciever.key(), GiftErrors::NotValidReciever);


        BurnV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            // .collection(Some(&self.collection.to_account_info()))
            .authority(Some(&self.reciever.to_account_info()))
            .payer(&self.reciever.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;



        Ok(())
        }
        
}       
