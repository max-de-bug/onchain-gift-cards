use anchor_lang::prelude::*;

use crate::errors::GiftCardError;
use crate::events::RuleSetUpdated;
use crate::state::GiftCard;

#[derive(Accounts)]
pub struct RuleSet<'info> {
    #[account(
        mut,
        seeds = [GiftCard::SEED_PREFIX, owner.key().as_ref()],
        bump = gift_card.bump,
    )]
    pub gift_card: Account<'info, GiftCard>,

    pub owner: Signer<'info>,
}

/// Sets the allowed merchants for a gift card
pub fn handler(ctx: Context<RuleSet>, allowed_merchants: Vec<Pubkey>) -> Result<()> {
    let gift_card = &mut ctx.accounts.gift_card;

    // Only owner can set allowed merchants
    require!(
        ctx.accounts.owner.key() == gift_card.owner,
        GiftCardError::Unauthorized
    );

    // Limit merchants to prevent account size issues
    require!(
        allowed_merchants.len() <= 10,
        GiftCardError::TooManyMerchants
    );

    gift_card.allowed_merchants = allowed_merchants.clone();

    emit!(RuleSetUpdated {
        gift_card: ctx.accounts.gift_card.key(),
        allowed_merchants,
    });

    Ok(())
}

