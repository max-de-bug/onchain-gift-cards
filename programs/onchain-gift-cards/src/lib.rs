use anchor_lang::prelude::*;

declare_id!("GcoSaa4P2NADPsf6R5urbrUEv9SccPTP5Xjd6GznV8p");

#[program]
pub mod onchain_gift_cards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
