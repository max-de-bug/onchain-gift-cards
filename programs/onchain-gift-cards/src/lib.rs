use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("GcoSaa4P2NADPsf6R5urbrUEv9SccPTP5Xjd6GznV8p");

#[program]
pub mod onchain_gift_cards {
    use super::*;

    /// Creates a new gift card with specified amount and dates
    pub fn create_gift_card(
        ctx: Context<CreateGiftCard>,
        card_id: u64,
        amount: u64,
        unlock_date: i64,
        refund_date: i64,
    ) -> Result<()> {
        instructions::create_gift_card::handler_create_gift_card(ctx, card_id, amount, unlock_date, refund_date)
    }

    /// Sets the allowed merchants for a gift card
    pub fn rule_set(ctx: Context<RuleSet>, card_id: u64, allowed_merchants: Vec<Pubkey>) -> Result<()> {
        instructions::rule_set::handler_rule_set(ctx, card_id, allowed_merchants)
    }

    /// Redeems tokens from a gift card to a merchant
    pub fn redeem(ctx: Context<Redeem>, card_id: u64, amount: u64) -> Result<()> {
        instructions::redeem::handler_redeem(ctx, card_id, amount)
    }

    /// Refunds remaining balance to the original gift giver after refund date
    pub fn refund(ctx: Context<Refund>, card_id: u64) -> Result<()> {
        instructions::refund::handler_refund(ctx, card_id)
    }

    /// Deletes an empty gift card account and reclaims rent
    pub fn delete_gift_card(ctx: Context<DeleteGiftCard>, card_id: u64) -> Result<()> {
        instructions::delete_gift_card::handler_delete_gift_card(ctx, card_id)
    }
}
