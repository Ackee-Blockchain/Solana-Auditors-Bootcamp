use anchor_lang::prelude::*;

pub fn _dummy_ix(_ctx: Context<DummyContext>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct DummyContext<'info> {
    pub signer: Signer<'info>,
}
