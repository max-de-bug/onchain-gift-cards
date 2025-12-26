use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::errors::GiftCardError;
use crate::events::GiftCardCreated;
use crate::state::GiftCard;

#[derive(Accounts)]
pub struct CreateGiftCard<'info> {
    #[account(
        init,
        payer = gift_giver,
        space = 8 + GiftCard::INIT_SPACE,
        seeds = [GiftCard::SEED_PREFIX, gift_giver.key().as_ref()],
        bump
    )]
    pub gift_card: Account<'info, GiftCard>,

    #[account(
        init,
        payer = gift_giver,
        associated_token::mint = token_mint,
        associated_token::authority = gift_card,
        associated_token::token_program = token_program,
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub gift_giver_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub gift_giver: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

/// Creates a new gift card with specified amount and dates
pub fn handler(
    ctx: Context<CreateGiftCard>,
    amount: u64,
    unlock_date: i64,
    refund_date: i64,
) -> Result<()> {
    let gift_card = &mut ctx.accounts.gift_card;
    let clock = Clock::get()?;

    // Validate dates
    require!(
        unlock_date >= clock.unix_timestamp,
        GiftCardError::InvalidUnlockDate
    );
    require!(
        refund_date > unlock_date,
        GiftCardError::InvalidRefundDate
    );

    // Store decimals for later transfers
    let decimals = ctx.accounts.token_mint.decimals;

    // Initialize gift card state
    gift_card.owner = ctx.accounts.gift_giver.key();
    gift_card.balance = amount;
    gift_card.unlock_date = unlock_date;
    gift_card.refund_date = refund_date;
    gift_card.bump = ctx.bumps.gift_card;
    gift_card.token_mint = ctx.accounts.token_mint.key();
    gift_card.escrow_token_account = ctx.accounts.escrow_token_account.key();
    gift_card.allowed_merchants = vec![];
    gift_card.decimals = decimals;

    // Transfer tokens from gift giver to escrow account
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.gift_giver_token_account.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.gift_giver.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

    emit!(GiftCardCreated {
        owner: gift_card.owner,
        balance: gift_card.balance,
        unlock_date: gift_card.unlock_date,
        refund_date: gift_card.refund_date,
    });

    Ok(())
}

