use anchor_lang::prelude::*;

#[event]
pub struct GiftCardCreated {
    pub owner: Pubkey,
    pub balance: u64,
    pub unlock_date: i64,
    pub refund_date: i64,
}

#[event]
pub struct GiftCardRedeemed {
    pub gift_card: Pubkey,
    pub merchant: Pubkey,
    pub amount: u64,
    pub remaining_balance: u64,
}

#[event]
pub struct RuleSetUpdated {
    pub gift_card: Pubkey,
    pub allowed_merchants: Vec<Pubkey>,
}

#[event]
pub struct BalanceRefunded {
    pub gift_card: Pubkey,
    pub amount: u64,
    pub remaining_balance: u64,
}

