use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub bump: u8,
    pub rewards_bump: u8,
    pub treasury_bump: u8,
    pub fee: u16,
    pub name: String, // Limit the str to 32 bytes 

}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 1+ 1 + 1 + 2*(4 + 32);
}