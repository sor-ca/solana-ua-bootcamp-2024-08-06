/*use anchor_lang::prelude::*;
use anchor_spl::token::{self, ApproveChecked, TokenAccount, Mint, Token, Transfer};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub maker_atk_account: Account<'info, TokenAccount>,
    pub maker_atk_mint: Account<'info, Mint>,
    #[account(init, payer = maker, space = 8 + 8 + 32 + 32 + 8)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct EscrowAccount {
    pub maker: Pubkey,
    pub maker_atk_amount: u64,
    pub maker_atk_mint: Pubkey,
    pub taker_btk_amount: u64,
    pub taker_btk_mint: Pubkey,
}

pub fn make_offer(
    ctx: Context<MakeOffer>,
    maker_atk_amount: u64,
    taker_btk_amount: u64,
    taker_btk_mint: Pubkey,
) -> Result<()> {
    // Store offer details in escrow account
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.maker = *ctx.accounts.maker.key;
    escrow_account.maker_atk_amount = maker_atk_amount;
    escrow_account.maker_atk_mint = ctx.accounts.maker_atk_mint.key();
    escrow_account.taker_btk_amount = taker_btk_amount;
    escrow_account.taker_btk_mint = taker_btk_mint;

    // Approve the program to spend the maker's ATK tokens
    let cpi_accounts = ApproveChecked {
        to: ctx.accounts.maker_atk_account.to_account_info().clone(),
        delegate: ctx.accounts.escrow_account.to_account_info().clone(),
        authority: ctx.accounts.maker.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::approve_checked(
        cpi_ctx,
        maker_atk_amount,
        ctx.accounts.maker_atk_mint.decimals,
    )?;

    Ok(())
}*/
use anchor_lang::prelude::*;
use anchor_spl::token::{self, ApproveChecked, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub maker_atk_account: Account<'info, TokenAccount>,
    pub maker_atk_mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = maker, space = 8 + EscrowAccount::LEN)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct EscrowAccount {
    pub maker: Pubkey,
    pub maker_atk_amount: u64,
    pub maker_atk_mint: Pubkey,
    pub taker_btk_amount: u64,
    pub taker_btk_mint: Pubkey,
}

impl EscrowAccount {
    const LEN: usize = 8 + 32 + 8 + 32 + 8 + 32;
}

// The make_offer function sets up an offer by storing the details in an EscrowAccount.
// It uses the approve_checked function to allow the program to transfer the specified amount of ATK tokens from Alice's account when the offer is accepted.
pub fn make_offer(
    ctx: Context<MakeOffer>,
    maker_atk_amount: u64,
    taker_btk_amount: u64,
    taker_btk_mint: Pubkey,
) -> Result<()> {
    // Store offer details in escrow account
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.maker = *ctx.accounts.maker.key;
    escrow_account.maker_atk_amount = maker_atk_amount;
    escrow_account.maker_atk_mint = ctx.accounts.maker_atk_mint.key();
    escrow_account.taker_btk_amount = taker_btk_amount;
    escrow_account.taker_btk_mint = taker_btk_mint;

    // Approve the program to spend the maker's ATK tokens
    // The ApproveChecked CPI (Cross-Program Invocation) requires the following fields:
    // to: The token account from which tokens will be approved for transfer.
    // delegate: The account that will be allowed to transfer tokens on behalf of the authority.
    // authority: The account that owns the token account and grants the approval.
    // mint: The mint of the token, which is required to verify the token's decimals.
    let cpi_accounts = ApproveChecked {
        to: ctx.accounts.maker_atk_account.to_account_info(),
        delegate: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
        mint: ctx.accounts.maker_atk_mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::approve_checked(
        cpi_ctx,
        maker_atk_amount,
        ctx.accounts.maker_atk_mint.decimals,
    )?;

    Ok(())
}
