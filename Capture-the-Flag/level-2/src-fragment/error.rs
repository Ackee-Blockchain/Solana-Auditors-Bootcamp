use anchor_lang::prelude::*;

#[error_code]
pub enum AtlantisError {
    #[msg("Explorear already died!")]
    ZombiesNotAllowed,
    #[msg("Not enough mana!")]
    NotEnoughMana,
    #[msg("You need to obtain secrets from the previous level first!")]
    IncorrectSecrets,
    #[msg("Two identical explorers detected!")]
    SameExplorersNotAllowed,
    #[msg("Not enough experience points to reveal the Secret!")]
    NotEnoughExperience,
    #[msg("Not enough monsters defeated to reveal the Secret!")]
    NotEnoughMonstersDefeated,
}
