#[error_code]
pub enum GiftCardError {
    #[msg("Invalid unlock date")]
    InvalidUnlockDate,
    #[msg("Invalid refund date")]
    InvalidRefundDate,
    #[msg("Gift card is locked")]
    GiftCardLocked,
    #[msg("Gift card has expired")]
    GiftCardExpired,
    #[msg("Merchant not allowed")]
    MerchantNotAllowed,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Too many merchants")]
    TooManyMerchants,
    #[msg("Refund not available")]
    RefundNotAvailable,
    #[msg("No balance to refund")]
    NoBalanceToRefund,
}