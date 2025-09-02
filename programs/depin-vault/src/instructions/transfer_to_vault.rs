use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Mint, 
    transfer_checked, 
    TokenAccount, 
    TokenInterface, 
    TransferChecked,
};

#[derive(Accounts)]
pub struct TransferToVault<'info> {
    pub from: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = from,
    )]
    pub from_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        address = Pubkey::from_str_const("7yjCjijhaK7pqC21rwuzmwKaG9B6horHa4qzALHjjGZz")
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> TransferToVault<'info> {
    pub fn transfer_to_vault(&self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.from_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.from.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        Ok(())
    }
}