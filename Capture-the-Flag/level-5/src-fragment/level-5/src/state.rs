use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub index: u32,
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub expiration_time: i64,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
