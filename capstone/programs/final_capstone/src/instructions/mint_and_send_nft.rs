use anchor_lang::prelude::*;

use crate::GiftConfig;
use mpl_core::{
    accounts::BaseCollectionV1, instructions:: CreateV2CpiBuilder, types::{Attribute, Attributes, Plugin, PluginAuthority, PluginAuthorityPair}, ID as MPL_CORE_PROGRAM_ID 
};

#[derive(Accounts)]
pub struct MintAndSendNft<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
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
    pub reciever: SystemAccount<'info>,
    #[account(
        seeds = [b"gift", payer.key().as_ref(), reciever.key().as_ref()],
        bump,
        constraint = gift_config.gift_nft_mint == asset.key(),
    )]
    pub gift_config: Account<'info, GiftConfig>,
}

impl <'info> MintAndSendNft<'info> {
    pub fn mint_and_send_nft(&mut self) -> Result<()> {

        let mut collection_plugin = vec![];

        let attribute_list: Vec<Attribute> = vec![
            Attribute {
                key: "Unpacked".to_string(),
                value: "false".to_string() 
            },
        ];

        collection_plugin.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list}),
            authority: Some(PluginAuthority::UpdateAuthority)
        });

        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        // .collection(Some(&self.collection.to_account_info()))
        .payer(&self.payer.to_account_info())
        .update_authority(Some(&self.reciever.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .name("Solwish NFT".to_string())
        .uri("https://devnet.irys.xyz/8JQLG1kKVeTj7LxTCj2PhmPW9LLSqFFxDv6sZJeRxgpj".to_string())
        .plugins(collection_plugin)
        .invoke()?;
        
        Ok(())
    }
}