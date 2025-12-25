#[event]
pub struct GiftCardCreated {
   pun owner: Pubkey,
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
pub struct RuleSet {
  pub gift_card: Pubkey,
  pub merchant: Pubkey,

}

#[event]
pub struct BalanceRefunded {
    pub gift_card: Pubkey,
    pub amount: u64,
    pub remaining_balance: u64,
}
