use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use super::EscrowAccount;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub taker_btk_account: Account<'info, TokenAccount>,
    pub taker_btk_mint: Account<'info, Mint>,
    // #[account(mut)]
    // pub maker: AccountInfo<'info>,
    /// CHECK: This is safe because we are only using it as a public key reference.
    #[account(mut)]
    pub maker: UncheckedAccount<'info>,
    #[account(mut)]
    pub maker_atk_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Transfer BTK tokens from the taker to the maker
    let cpi_accounts = Transfer {
        from: ctx.accounts.taker_btk_account.to_account_info(),
        to: ctx.accounts.maker_atk_account.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, ctx.accounts.escrow_account.taker_btk_amount)?;

    // Transfer ATK tokens from the maker to the taker
    let cpi_accounts = Transfer {
        from: ctx.accounts.maker_atk_account.to_account_info(),
        to: ctx.accounts.taker_btk_account.to_account_info(),
        authority: ctx.accounts.escrow_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, ctx.accounts.escrow_account.maker_atk_amount)?;

    // Close the escrow account
    let escrow_account = ctx.accounts.escrow_account.to_account_info();
    **ctx.accounts.maker.lamports.borrow_mut() += **escrow_account.lamports.borrow();
    **escrow_account.lamports.borrow_mut() = 0;
    *escrow_account.try_borrow_mut_data()? = &mut [];

    Ok(())
}
