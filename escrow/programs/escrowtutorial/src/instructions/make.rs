use anchor_spl::{
    associated_token::AssociatedToken, mint,
    token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked}, token_interface::TokenInterface
};
use crate::state::Escrow;
use anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction, system_program::Transfer};






#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make <'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, TokenInterface>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

}

impl <'info> Make <'info> {
    pub fn initialize_escrow(&mut self, seed: u64, recieve: u64, bumps: &MakeBumps) ->Result<()>{
        self.escrow.seed = seed;

        self.escrow.set_inner(Escrow {
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            recieve,
            seed,
            bump: bumps.escrow,
    });

        Ok(())
    }
    pub fn tranfer_mint_a_to_vault(&mut self, deposit: u64) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let accounts = TransferChecked {
         from: self.maker_ata_a.to_account_info(),
         to: self.vault.to_account_info(),
         authority: self.maker.to_account_info(),
         mint: self.mint_a.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, accounts);

        transfer_checked(cpi_context, deposit, self.mint_a.decimals)?;
        // transfer(self.associated_token_program, self.maker_ata_a, self.vault, deposit, self.maker, self.escrow.recieve);

        Ok(())
    }
}