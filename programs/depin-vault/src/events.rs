use anchor_lang::prelude::*;

#[event]
pub struct TempSetEvent {
    pub old_value: u32,
    pub new_value: u32,
}

#[event]
pub struct TransferEvent {
    pub amount: u64,
    pub vault_balance: u64,
}