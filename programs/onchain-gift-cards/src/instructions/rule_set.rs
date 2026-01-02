use anchor_lang::prelude::*;

use crate::errors::GiftCardError;
use crate::events::RuleSetUpdated;
use crate::state::GiftCard;

#[derive(Accounts)]
#[instruction(card_id: u64)]
pub struct RuleSet<'info> {
    #[account(
        mut,
        seeds = [GiftCard::SEED_PREFIX, owner.key().as_ref(), &card_id.to_le_bytes()],
        bump = gift_card.bump,
        constraint = gift_card.card_id == card_id @ GiftCardError::InvalidCardId,
    )]
    pub gift_card: Account<'info, GiftCard>,

    pub owner: Signer<'info>,
}

/// Sets the allowed merchants for a gift card
pub fn handler_rule_set(ctx: Context<RuleSet>, card_id: u64, allowed_merchants: Vec<Pubkey>) -> Result<()> {
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
        card_id,
        gift_card: ctx.accounts.gift_card.key(),
        allowed_merchants,
    });

    Ok(())
}
