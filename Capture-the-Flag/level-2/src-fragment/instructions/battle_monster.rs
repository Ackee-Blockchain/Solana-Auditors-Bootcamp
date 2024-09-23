use anchor_lang::prelude::*;

use crate::{error::AtlantisError, state::Explorer};

pub const FIGHT_EXPERIENCE: u8 = 5;
pub const FIGHT_INJURY: u8 = 40;

// Battles a monster and reduces explorer's health, increases experience if not dead
pub fn _battle_monster(ctx: Context<BattleMonster>) -> Result<()> {
    let explorer_account = &mut ctx.accounts.explorer_account;

    require!(
        explorer_account.health > 0,
        AtlantisError::ZombiesNotAllowed
    );

    // Reduce health after the battle
    explorer_account.health = explorer_account.health.saturating_sub(FIGHT_INJURY);
    explorer_account.experience = explorer_account.experience.saturating_add(FIGHT_EXPERIENCE);
    explorer_account.monsters_defeated = explorer_account.monsters_defeated.saturating_add(1);

    Ok(())
}

#[derive(Accounts)]
pub struct BattleMonster<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"EXPLORER", user.key().as_ref()],
        bump
    )]
    pub explorer_account: Account<'info, Explorer>,
}
