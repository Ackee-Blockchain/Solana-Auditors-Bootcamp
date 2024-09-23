use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

use anchor_lang::solana_program::hash::hash;

declare_id!("5zjbNpnsSkCNG6zHzK183ujm6dn6fWeHWeUnk1Rzrs1Y");

#[program]
pub mod level_3 {

    use super::*;

    pub fn initialize(ctx: Context<InitializeFaction>, name: String, symbol: String) -> Result<()> {
        _initialize_faction(ctx, name, symbol)
    }
    pub fn obtain_faction_token(ctx: Context<ObtainFactionToken>) -> Result<()> {
        _obtain_faction_token(ctx)
    }
    pub fn show_faction_secret(ctx: Context<ShowFactionSecret>) -> Result<()> {
        let secret = hash(secret.as_bytes()).to_string();
        if secret.ne("DBt7KP4YFHmEGKCqKVDdYDDaqLdYhd3CnqkEQpKWdrBT") {
            return err!(Level3Error::IncorrectSecret);
        }
        _show_faction_details(ctx)
    }
}
