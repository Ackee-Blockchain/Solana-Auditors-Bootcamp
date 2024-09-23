use anchor_lang::prelude::*;

#[account]
pub struct Faction {
    pub authority: Pubkey,
    pub members_count: u64,
    pub mint: Pubkey,
    pub bump: u8,
}

impl Faction {
    pub const LEN: usize = 32 + 8 + 32 + 1;
}
