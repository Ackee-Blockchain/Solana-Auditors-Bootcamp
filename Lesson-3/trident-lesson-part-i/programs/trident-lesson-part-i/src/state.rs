use anchor_lang::prelude::*;

#[account]
pub struct Asset {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub counter: u64,
}

impl Asset {
    pub const LEN: usize = 32 + 32 + 8;
}
