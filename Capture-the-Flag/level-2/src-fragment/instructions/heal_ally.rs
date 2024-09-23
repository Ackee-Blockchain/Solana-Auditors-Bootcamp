use anchor_lang::prelude::*;

use crate::{error::AtlantisError, state::Explorer};

pub const HEAL_EXPERIENCE: u8 = 5;

// Heal another explorer using your mana and gain experience
pub fn _heal_ally(ctx: Context<HealAlly>) -> Result<()> {
    let healer = &mut ctx.accounts.healer;
    let injured_explorer = &mut ctx.accounts.injured_explorer;

    require!(
        injured_explorer.health > 0,
        AtlantisError::ZombiesNotAllowed
    );
    require!(healer.health > 0, AtlantisError::ZombiesNotAllowed);

    let to_heal = 100 - injured_explorer.health;

    require!(healer.mana >= to_heal, AtlantisError::NotEnoughMana);

    injured_explorer.health = 100;
    healer.mana -= to_heal;
    healer.experience = healer.experience.saturating_add(HEAL_EXPERIENCE);

    Ok(())
}

#[derive(Accounts)]
pub struct HealAlly<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub injured_explorer: Account<'info, Explorer>,

    #[account(
        mut,
        seeds = [b"EXPLORER", user.key().as_ref()],
        bump
    )]
    pub healer: Account<'info, Explorer>,
}
