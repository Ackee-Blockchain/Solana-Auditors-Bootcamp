use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub mint: Pubkey,
    pub open_time: i64,
    pub vault_content: u8,
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 32 + 8 + 1 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintParameters {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[event]
pub struct ReadEvent {
    pub vault_content: u8,
}
