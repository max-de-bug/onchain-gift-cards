use anchor_lang::prelude::*;

use crate::errors::GiftCardError;
use crate::events::GiftCardDeleted;
use crate::state::GiftCard;

#[derive(Accounts)]
#[instruction(card_id: u64)]
pub struct DeleteGiftCard<'info> {
    #[account(
        mut,
        seeds = [GiftCard::SEED_PREFIX, owner.key().as_ref(), &card_id.to_le_bytes()],
        bump = gift_card.bump,
        constraint = gift_card.card_id == card_id @ GiftCardError::InvalidCardId,
        constraint = gift_card.owner == owner.key() @ GiftCardError::Unauthorized,
        constraint = gift_card.balance == 0 @ GiftCardError::HasBalance,
        close = owner // Closes the account and sends rent to owner
    )]
    pub gift_card: Account<'info, GiftCard>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

/// Deletes a gift card account and reclaims rent
/// Only works when balance is 0
pub fn handler_delete_gift_card(ctx: Context<DeleteGiftCard>, _card_id: u64) -> Result<()> {
    let card_id = ctx.accounts.gift_card.card_id;

    // Emit event before account is closed
    emit!(GiftCardDeleted {
        card_id,
    });

    // Account closure is handled automatically by Anchor's `close` constraint
    Ok(())
}
