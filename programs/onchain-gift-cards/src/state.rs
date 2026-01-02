use anchor_lang::prelude::*;

/// Gift Card account state
#[account]
#[derive(InitSpace)]
pub struct GiftCard {
    /// Unique card identifier (allows multiple cards per owner)
    pub card_id: u64,
    /// The original creator/owner of the gift card
    pub owner: Pubkey,
    /// Current balance in token units
    pub balance: u64,
    /// Unix timestamp when the gift card becomes redeemable
    pub unlock_date: i64,
    /// Unix timestamp after which the gift card can be refunded
    pub refund_date: i64,
    /// The SPL token mint for this gift card
    pub token_mint: Pubkey,
    /// The escrow token account holding the funds
    pub escrow_token_account: Pubkey,
    /// PDA bump seed
    pub bump: u8,
    /// Token decimals for transfer_checked
    pub decimals: u8,
    /// List of allowed merchant pubkeys (empty = all merchants allowed)
    #[max_len(10)]
    pub allowed_merchants: Vec<Pubkey>,
}

impl GiftCard {
    /// Seed prefix for deriving gift card PDA
    pub const SEED_PREFIX: &'static [u8] = b"gift_card";
}
