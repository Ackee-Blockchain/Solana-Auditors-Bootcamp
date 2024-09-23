use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hashv;

declare_id!("8uxc3JNoZ5FqhZd3mTMZMiFmwpup6uMreKgRGTsPhqoX");

pub const FIGHT_EXPERIENCE: u8 = 5;
pub const FIGHT_INJURY: u8 = 40;
pub const HEAL_EXPERIENCE: u8 = 5;

mod error;
mod instructions;
mod state;

use crate::instructions::*;
use error::AtlantisError;

#[program]
pub mod level_2 {

    use super::*;

    // Initializes a new explorer
    pub fn init_explorer(
        ctx: Context<InitExplorer>,
        secret1: String,
        secret2: String,
    ) -> Result<()> {
        let secrets = hashv(&[secret1.as_bytes(), secret2.as_bytes()]).to_string();
        if secrets.ne("497HQPU19duDXfWTFvXb2PZNCgnMcGRqLxuLXjWNkkii") {
            return err!(AtlantisError::IncorrectSecrets);
        }

        _init_explorer(ctx)?;

        Ok(())
    }

    // Battles a monster and reduces explorer's health, increases experience if not dead
    pub fn battle_monster(ctx: Context<BattleMonster>) -> Result<()> {
        _battle_monster(ctx)?;

        Ok(())
    }

    // Heal another explorer using your mana and gain experience
    pub fn heal_ally(ctx: Context<HealAlly>) -> Result<()> {
        _heal_ally(ctx)?;

        Ok(())
    }
    pub fn reveal_secret(_ctx: Context<RevealSecret>) -> Result<()> {
        // Nothing to see here.
        Ok(())
    }
}
