#[derive(Accounts)]
pub struct CreateGiftCard<'info> {
    #[account(
        init,
        payer = gift_giver,
        space = 8 + GiftCard::INIT_SPACE,
        seeds = [b"gift_card", gift_giver.key().as_ref()],
        bump
    )]
    pub gift_card: Account<'info, GiftCard>,

    #[account(
        init_if_needed,
        payer = gift_giver,
        associated_token::mint = token_mint,
        associated_token::authority = gift_card,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub gift_giver_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub gift_giver: Signer<'info>,

    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedTokenProgram>,
}

#[derive(Accounts)]
pub struct RuleSet<'info> {
    #[account(mut)]
    pub gift_card: Account<'info, GiftCard>,

    pub owner: Signer<'info>,
}


#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub gift_card: Account<'info, GiftCard>,

    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub merchant_token_account: Account<'info, TokenAccount>,

    pub merchant: UncheckedAccount<'info>,

    pub owner: Signer<'info>,

    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub gift_card: Account<'info, GiftCard>,

    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub gift_giver_token_account: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,

    pub token_program: Program<'info, token::Token>,
}

#[account]
#[derive(InitSpace)]
pub struct GiftCard {
    pub owner: Pubkey,
    pub balance: u64,
    pub unlock_date: i64,
    pub refund_date: i64,
    pub token_mint: Pubkey,
    pub escrow_token_account: Pubkey,
    pub bump: u8,
    #[max_len(10)]
    pub allowed_merchants: Vec<Pubkey>,
}