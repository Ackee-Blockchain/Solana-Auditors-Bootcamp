// There are a lot of secret here

use crate::state::Faction;
#[derive(Accounts)]
pub struct ShowFactionSecret<'info> {
    pub faction_member: Signer<'info>,
    #[account(
        seeds = [b"faction",faction.authority.key().as_ref(),faction.mint.key().as_ref()],
        bump
    )]
    pub faction: Account<'info, Faction>,
    #[account(
        token::authority = faction_member,
        token::mint = mint,
        constraint = member_token_account.amount == 1
    )]
    pub member_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mint::decimals = 0,
        mint::authority = faction,
        mint::freeze_authority = faction,
        extensions::metadata_pointer::metadata_address = mint.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
}
