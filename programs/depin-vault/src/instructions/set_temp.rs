use anchor_lang::prelude::*;

use crate::state::Temp;

#[derive(Accounts)]
pub struct SetTemp<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Temp::INIT_SPACE,
        seeds = [b"temp", user.key().as_ref()],
        bump,
    )]
    pub temp: Account<'info, Temp>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetTemp<'info> {
    pub fn set_temp(&mut self, value: u32) -> Result<()> {
        let temp = &mut self.temp;
        temp.value = value;
        temp.latest_update = Clock::get()?.unix_timestamp;
        Ok(())
    }
}