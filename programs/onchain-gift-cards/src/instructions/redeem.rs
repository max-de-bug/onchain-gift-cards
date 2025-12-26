use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::errors::GiftCardError;
use crate::events::GiftCardRedeemed;
use crate::state::GiftCard;

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(
        mut,
        seeds = [GiftCard::SEED_PREFIX, gift_card.owner.as_ref()],
        bump = gift_card.bump,
    )]
    pub gift_card: Account<'info, GiftCard>,

    #[account(
        mut,
        constraint = escrow_token_account.key() == gift_card.escrow_token_account @ GiftCardError::Unauthorized,
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub merchant_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Merchant is validated against allowed_merchants list in the instruction
    pub merchant: UncheckedAccount<'info>,

    pub owner: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Redeems tokens from a gift card to a merchant
pub fn handler(ctx: Context<Redeem>, amount: u64) -> Result<()> {
    let gift_card = &ctx.accounts.gift_card;
    let clock = Clock::get()?;

    // Check if gift card is unlocked
    require!(
        clock.unix_timestamp >= gift_card.unlock_date,
        GiftCardError::GiftCardLocked
    );

    // Check if gift card has not expired
    require!(
        clock.unix_timestamp < gift_card.refund_date,
        GiftCardError::GiftCardExpired
    );

    // Check sufficient balance
    require!(
        gift_card.balance >= amount,
        GiftCardError::InsufficientBalance
    );

    // Check if merchant is allowed (empty list = all merchants allowed)
    require!(
        gift_card.allowed_merchants.is_empty()
            || gift_card.allowed_merchants.contains(&ctx.accounts.merchant.key()),
        GiftCardError::MerchantNotAllowed
    );

    // Store values needed for seeds and event
    let owner_key = gift_card.owner;
    let bump = gift_card.bump;
    let decimals = gift_card.decimals;
    let gift_card_key = ctx.accounts.gift_card.key();
    let new_balance = gift_card.balance - amount;

    // Create signer seeds for PDA (gift_card is authority over escrow)
    let seeds = &[
        GiftCard::SEED_PREFIX,
        owner_key.as_ref(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Transfer tokens from escrow to merchant
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.merchant_token_account.to_account_info(),
        authority: ctx.accounts.gift_card.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

    // Update balance after successful transfer
    ctx.accounts.gift_card.balance = new_balance;

    emit!(GiftCardRedeemed {
        gift_card: gift_card_key,
        merchant: ctx.accounts.merchant.key(),
        amount,
        remaining_balance: new_balance,
    });

    Ok(())
}

