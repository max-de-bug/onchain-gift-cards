use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::errors::GiftCardError;
use crate::events::BalanceRefunded;
use crate::state::GiftCard;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        seeds = [GiftCard::SEED_PREFIX, owner.key().as_ref()],
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
    pub gift_giver_token_account: InterfaceAccount<'info, TokenAccount>,

    pub owner: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Refunds remaining balance to the original gift giver after refund date
pub fn handler(ctx: Context<Refund>) -> Result<()> {
    let gift_card = &ctx.accounts.gift_card;
    let clock = Clock::get()?;

    // Only owner can request refund
    require!(
        ctx.accounts.owner.key() == gift_card.owner,
        GiftCardError::Unauthorized
    );

    // Check if refund period has started
    require!(
        clock.unix_timestamp >= gift_card.refund_date,
        GiftCardError::RefundNotAvailable
    );

    // Check if there's balance to refund
    let refund_amount = gift_card.balance;
    require!(refund_amount > 0, GiftCardError::NoBalanceToRefund);

    // Store values needed for seeds and event
    let owner_key = gift_card.owner;
    let bump = gift_card.bump;
    let decimals = gift_card.decimals;
    let gift_card_key = ctx.accounts.gift_card.key();

    // Create signer seeds for PDA (gift_card is authority over escrow)
    let seeds = &[
        GiftCard::SEED_PREFIX,
        owner_key.as_ref(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Transfer all remaining tokens from escrow back to gift giver
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.gift_giver_token_account.to_account_info(),
        authority: ctx.accounts.gift_card.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token_interface::transfer_checked(cpi_ctx, refund_amount, decimals)?;

    // Set balance to 0 after full refund
    ctx.accounts.gift_card.balance = 0;

    emit!(BalanceRefunded {
        gift_card: gift_card_key,
        amount: refund_amount,
        remaining_balance: 0,
    });

    Ok(())
}

