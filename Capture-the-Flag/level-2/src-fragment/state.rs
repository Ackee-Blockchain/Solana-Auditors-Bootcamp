use anchor_lang::prelude::*;

#[account]
pub struct Explorer {
    pub mana: u8,
    pub health: u8,
    pub experience: u8,
    pub monsters_defeated: u8,
}

impl Explorer {
    pub const LEN: usize = 1 + 1 + 1 + 1;
}
