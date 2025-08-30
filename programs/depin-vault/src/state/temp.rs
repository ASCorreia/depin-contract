use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Temp {
    pub value: u8,
    pub latest_update: i64,
    pub bump: u8,
}