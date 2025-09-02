#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod state;
mod instructions;

use instructions::*;

declare_id!("3iVjkQPbzHRfNGqzkrNBfE1m2TJYWQbycCMPukrdk6pP");

#[program]
pub mod depin_vault {
    use super::*;

    pub fn transfer_to_vault(ctx: Context<TransferToVault>) -> Result<()> {
        ctx.accounts.transfer_to_vault(10000000)
    }

    pub fn set_temp(ctx: Context<SetTemp>, value: u32) -> Result<()> {
        ctx.accounts.set_temp(value)
    }
}
