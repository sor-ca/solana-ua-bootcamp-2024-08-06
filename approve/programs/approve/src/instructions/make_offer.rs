use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token::{self, approve, Approve, Mint, TokenAccount},
//     token_interface::TokenInterface,
// };
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{approve, Approve};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub atk_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub btk_mint: InterfaceAccount<'info, Mint>,

    //#[account(mut)]
    #[account(
        mut,
        associated_token::mint = atk_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_atk_account: InterfaceAccount<'info, TokenAccount>,
    #[account(init, payer = maker, space = crate::constants::ANCHOR_DISCRIMINATOR + EscrowAccount::INIT_SPACE)]
    pub escrow_account: Account<'info, EscrowAccount>,

    //pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    //pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    pub maker: Pubkey,
    pub maker_atk_amount: u64,
    pub atk_mint: Pubkey,
    pub taker_btk_amount: u64,
    pub btk_mint: Pubkey,
}

// pub struct Approve<'info> {
//     pub to: AccountInfo<'info>,
//     pub delegate: AccountInfo<'info>,
//     pub authority: AccountInfo<'info>,
// }

// The make_offer function sets up an offer by storing the details in an EscrowAccount.
// It uses the approve_checked function to allow the program to transfer the specified amount of ATK tokens from Alice's account when the offer is accepted.
pub fn make_offer(
    ctx: Context<MakeOffer>,
    maker_atk_amount: u64,
    taker_btk_amount: u64,
) -> Result<()> {
    msg!("Starting to make offer...");
    // Store offer details in escrow account
    // let escrow_account = &mut ctx.accounts.escrow_account;
    // escrow_account.maker = *ctx.accounts.maker.key;
    // escrow_account.maker_atk_amount = maker_atk_amount;
    // escrow_account.atk_mint = ctx.accounts.atk_mint.key();
    // escrow_account.taker_btk_amount = taker_btk_amount;
    // escrow_account.btk_mint = btk_mint;
    ctx.accounts.escrow_account.set_inner(EscrowAccount {
        maker: ctx.accounts.maker.key(),
        atk_mint: ctx.accounts.atk_mint.key(),
        btk_mint: ctx.accounts.btk_mint.key(),
        maker_atk_amount,
        taker_btk_amount,
    });

    // Approve the program to spend the maker's ATK tokens
    // The ApproveChecked CPI (Cross-Program Invocation) requires the following fields:
    // to: The token account from which tokens will be approved for transfer.
    // delegate: The account that will be allowed to transfer tokens on behalf of the authority.
    // authority: The account that owns the token account and grants the approval.
    // mint: The mint of the token, which is required to verify the token's decimals.
    // let cpi_accounts = ApproveChecked {
    //     to: ctx.accounts.maker_atk_account.to_account_info(),
    //     delegate: ctx.accounts.escrow_account.to_account_info(),
    //     authority: ctx.accounts.maker.to_account_info(),
    //     mint: ctx.accounts.atk_mint.to_account_info(),
    // };
    let cpi_accounts = Approve {
        to: ctx.accounts.maker_atk_account.to_account_info(),
        delegate: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    if let Err(e) =
        //token::approve_checked(cpi_ctx, maker_atk_amount, ctx.accounts.atk_mint.decimals)
        approve(cpi_ctx, maker_atk_amount)
    {
        msg!("Error approving tokens: {:?}", e);
        return Err(e);
    }

    Ok(())
}
