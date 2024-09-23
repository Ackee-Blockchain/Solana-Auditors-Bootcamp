use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct GuardianSet {
    pub index: u32,
    pub escrow: Pubkey,
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub guardian_signatures: Vec<[u8; 32]>,
    pub bump: u8,
}

impl GuardianSet {
    pub(crate) fn compute_size(num_signatures: usize) -> usize {
        4 + 32 + 32 + 32 + 1 + 4 + num_signatures * 32
    }
}
