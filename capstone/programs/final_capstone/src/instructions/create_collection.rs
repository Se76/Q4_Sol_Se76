use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    types::{PluginAuthorityPair, Plugin, PermanentFreezeDelegate}, 
    instructions::CreateCollectionV2CpiBuilder, 
};

#[derive(Accounts)]
pub struct CollectionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,

}

impl <'info> CollectionAccounts<'info> {
    pub fn create_collection(&mut self) -> Result<()> { 
        let mut collection_plugins = vec![];

        collection_plugins.push( PluginAuthorityPair { plugin: Plugin::PermanentFreezeDelegate( PermanentFreezeDelegate { frozen: true}), authority: None});

        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
        .collection(&self.collection.to_account_info())
        .payer(&self.payer.to_account_info())
        .system_program(&self.system_program.to_account_info())
        // .update_authority(Some(&self.reputation_config.to_account_info()))
        .name("Official Solwish Gift Collection".to_string())
        .uri("https://devnet.irys.xyz/4CGdHEQCM2ETBbntBFdcaRbSZWGxTjTdyPasUNGy9gBT".to_string())
        .plugins(collection_plugins)
        .invoke()?;
        Ok(())
    }
}