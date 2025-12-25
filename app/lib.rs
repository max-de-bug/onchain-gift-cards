use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("GcoSaa4P2NADPsf6R5urbrUEv9SccPTP5Xjd6GznV8p");

#[progrm]

pub mod gift_card_problem {
    use super::*;

    pub fn create_gift_card(ctx: Context<CreateGiftCard>, amount: u64, unlock_date: i64, refund_date: i64) -> Result<()> {
        let gift_card = &mut ctx.accounts.gift_card;
        let clock = Clock::get()?;


        require!(
            unlock_date >= clock.unix_timestamp,
            GiftCardError::InvalidUnlockDate
        );
        require!(
            refund_date > unlock_date,
            GiftCardError::InvalidRefundDate
        );


        gift_card.owner = ctx.accounts.gift_giver.key();
        gift_card.balance = amount;
        gift_card.unlock_date = unlock_date;
        gift_card.refund_date = refund_date;
        gift_card.bump = *ctx.bumps.get("gift_card").unwrap();
        gift.card.token_mint = ctx.accounts.token_mint.key();
        gift_card.escrow_token_account = ctx.accounts.escrow_token_account.key();


      let cpi_A
      let cpi_accounts = Transfer {
        from: ctx.accounts.gift_giver_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.gift_giver.to_account_info(),
      };
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
      token::transfer(cpi_ctx, amount)?;
      
      emit!(GiftCardCreated {
        owner: gift_card.owner,
        balance: gift_card.balance,
        unlock_date: gift_card.unlock_date,
        refund_date: gift_card.refund_date,
      });
      Ok(())

    }

    pub fn rule_set(ctx: Context<RuleSet>, allowed_merchants: Vec<Pubkey>) -> Result<()> {
      let gift_card = &mut ctx.accounts.gift_card;
      require! (
        ctx.accounts.owner.key() == gift_card.owner,
        GiftCardError::Unauthorized
      );
    
      require! (
        allowed_merchants.len() <= 10,
        GiftCardError::TooManyMerchants
      );
      gift_card.allowed_merchants = allowed_merchants;
      emit!(RuleSet {
        gift_card: gift_card.key(),
        allowed_merchants: gift_card.allowed_merchants,
      });
      Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
      let gift_card = &mut ctx.accounts.gift_card;
      let clock = Clock::get()?;
      require!(
        clock.unix_timestamp >= gift_card.unlock_date,
        GiftCardError::GiftCardLocked);


      require! (
        gift_card.balance >= amount,
        GiftCardError::InsufficientBalance
      );

      require! (
        gift_card.allowed_merchants.contains(&ctx.accounts.merchant.key()),
        GiftCardError::MerchantNotAllowed
      )
   
      require! (
        clock.unix_timestamp < gift_card.refund_date,
        GiftCardError::GiftCardExpired
      );
  

      gift_card.balance -= amount;

      let seeds = &[
        b"gift_card",
        &ctx.accounts.owner.key().to_bytes(),
        &[gift_card.bump],
    ];
    let signer_seeds = &[&seeds[..]];

      let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.merchant_token_account.to_account_info(),
        authority: ctx.accounts.merchant.to_account_info(),
      };
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
      token::transfer(cpi_ctx, amount)?;

      emit!(GiftCardRedeemed {
        gift_card: gift_card.key(),
        merchant: ctx.accounts.merchant.key(),
        amount: amount,
        remaining_balance: gift_card.balance - amount,
      });
      Ok(())
    }

    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
      let gift_card = &mut ctx.accounts.gift_card;
      let clock = Clock::get()?;
       
      require! (
        ctx.accounts.owner.key() == gift_card.owner,
        GiftCardError::Unauthorized
      );
      require! (
        clock.unix_timestamp >= gift_card.refund_date,
        GiftCardError::RefundNotAvailable
      );

      require! (
        gift_card.balance >= amount,
        GiftCardError::NoBalanceToRefund
      );

      gift_card.balance = 0; // why 0?
      let seeds = &[ // why seeds?
        b"gift_card",
        &ctx.accounts.owner.key().to_bytes(),
        &[gift_card.bump], 
    ];
    let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.gift_giver_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        emit!(BalanceRefunded {
            gift_card: gift_card.key(),
            amount: amount,
            remaining_balance: gift_card.balance,
        });
        Ok(())
    }