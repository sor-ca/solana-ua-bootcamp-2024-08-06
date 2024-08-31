use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token::{self, Mint, TokenAccount, Transfer},
//     token_interface::TokenInterface,
// };

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use super::EscrowAccount;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub atk_mint: Box<InterfaceAccount<'info, Mint>>,
    pub btk_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = btk_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_btk_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = atk_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_atk_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = atk_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_atk_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = btk_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_btk_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Transfer BTK tokens from the taker to the maker
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.taker_btk_account.to_account_info(),
        to: ctx.accounts.maker_btk_account.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
        mint: ctx.accounts.btk_mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    if let Err(e) = transfer_checked(
        cpi_ctx,
        ctx.accounts.escrow_account.taker_btk_amount,
        ctx.accounts.btk_mint.decimals,
    ) {
        msg!("Error transferring BTK tokens: {:?}", e);
        return Err(e);
    }

    // Transfer ATK tokens from the maker to the taker
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.maker_atk_account.to_account_info(),
        to: ctx.accounts.taker_atk_account.to_account_info(),
        authority: ctx.accounts.escrow_account.to_account_info(),
        mint: ctx.accounts.atk_mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //token::transfer(cpi_ctx, ctx.accounts.escrow_account.maker_atk_amount)?;

    if let Err(e) = transfer_checked(
        cpi_ctx,
        ctx.accounts.escrow_account.maker_atk_amount,
        ctx.accounts.atk_mint.decimals,
    ) {
        msg!("Error transferring ATK tokens: {:?}", e);
        return Err(e);
    }

    msg!("Offer taken successfully.");

    // Close the escrow account
    let escrow_account = ctx.accounts.escrow_account.to_account_info();
    **ctx.accounts.maker.lamports.borrow_mut() += **escrow_account.lamports.borrow();
    **escrow_account.lamports.borrow_mut() = 0;
    *escrow_account.try_borrow_mut_data()? = &mut [];
    Ok(())
}
