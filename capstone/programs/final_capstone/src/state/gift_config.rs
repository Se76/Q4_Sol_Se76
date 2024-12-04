use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GiftConfig {
    pub creator: Pubkey, 
    pub reciever: Pubkey, 
    
    pub gift_nft_mint: Pubkey, 

    #[max_len(280)]
    pub greetings: String, 
    pub bump: u8, 
    #[max_len(20)]
    pub mints: Vec<Pubkey>, 
}
