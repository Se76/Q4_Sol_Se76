use anchor_lang::prelude::*;
use anchor_spl::{token:: Token, associated_token::AssociatedToken, metadata::Metadata};
use crate::GiftConfig;
use crate::GiftErrors;

#[derive(Accounts)]
// #[instruction(mints: Vec<Pubkey>, counter: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    pub reciever: SystemAccount<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [b"gift", creator.key().as_ref(), reciever.key().as_ref()],
        bump,
        space = GiftConfig::INIT_SPACE + 8,
    )]
    pub gift_config: Account<'info, GiftConfig>,

    #[account(mut)]
    pub asset: SystemAccount<'info>, // PLEASE IT SHOULD BE SIGNER FOR THE COLLECTION MINT NFT, CHECK MY-LEO PROJECT

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        greetings: String,
        bumps: &InitializeBumps
    ) -> Result<()> {
        // msg!("{}", greetings);

        require!(greetings.len() > 0 && greetings.len() < 280, GiftErrors::GreetingsAreTooLong);
        // require!(mints.len() > 0 && mints.len() <= 20, VectorOfMintsIsTooLong);

        self.gift_config.set_inner(GiftConfig{
            creator: self.creator.key(),
            reciever: self.reciever.key(),
            gift_nft_mint: self.asset.key(),
            greetings: greetings,
            bump: bumps.gift_config,
            mints: Vec::new(),
        });
        
        Ok(())
    }
}