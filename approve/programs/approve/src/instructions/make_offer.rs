use anchor_lang::prelude::*;
use anchor_spl::token::{self, ApproveChecked, Mint, Token, TokenAccount};

#[derive(Accounts)]
//#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub maker_atk_account: Account<'info, TokenAccount>,
    // #[account(
    //     mut,
    //     associated_token::mint = atk_mint,
    //     associated_token::authority = maker,
    //     associated_token::token_program = token_program
    // )]
    //pub maker_atk_account: InterfaceAccount<'info, TokenAccount>,
    // #[account(mint::token_program = token_program)]
    // pub atk_mint: InterfaceAccount<'info, Mint>,
    // #[account(mint::token_program = token_program)]
    // pub btk_mint: InterfaceAccount<'info, Mint>,
    pub atk_mint: Account<'info, Mint>,
    pub btk_mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = maker, space = crate::constants::ANCHOR_DISCRIMINATOR + EscrowAccount::INIT_SPACE)]
    pub escrow_account: Account<'info, EscrowAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    //pub id: u64,
    pub maker: Pubkey,
    pub maker_atk_amount: u64,
    pub atk_mint: Pubkey,
    pub taker_btk_amount: u64,
    pub btk_mint: Pubkey,
    //pub bump: u8,
}

// impl EscrowAccount {
//     const LEN: usize = 8 + 32 + 8 + 32 + 8 + 32;
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
    let cpi_accounts = ApproveChecked {
        to: ctx.accounts.maker_atk_account.to_account_info(),
        delegate: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
        mint: ctx.accounts.atk_mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    if let Err(e) =
        token::approve_checked(cpi_ctx, maker_atk_amount, ctx.accounts.atk_mint.decimals)
    {
        msg!("Error approving tokens: {:?}", e);
        return Err(e);
    }

    Ok(())
}
/*use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub amount_atk: u64,
    pub amount_btk: u64,
    pub mint_btk: Pubkey,
    pub maker_btk_token_account: Pubkey,
    pub taker_atk_token_account: Pubkey,
}

//use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, ApproveChecked, CreateAssociatedTokenAccount, Mint, Token, TokenAccount,
};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub maker_atk_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub maker_atk_mint: Account<'info, Mint>,
    #[account(init, payer = maker, space = 8 + 32)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, Token>,
}

pub fn make_offer(
    ctx: Context<MakeOffer>,
    amount_atk: u64,
    amount_btk: u64,
    mint_btk: Pubkey,
) -> Result<()> {
    msg!("Starting to make offer...");

    // Create maker_btk_token_account (for Alice)
    let maker_btk_token_account = CreateAssociatedTokenAccount {
        payer: ctx.accounts.maker.to_account_info(),
        associated_token: ctx.accounts.escrow_account.to_account_info(), // escrow will hold the BTK account
        authority: ctx.accounts.maker.to_account_info(),
        mint: ctx.accounts.mint_btk.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };

    let cpi_program = ctx.accounts.associated_token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, maker_btk_token_account);
    if let Err(e) = token::create_associated_token_account(cpi_ctx) {
        msg!("Error creating maker BTK token account: {:?}", e);
        return Err(e);
    }

    // Approve ATK tokens
    let cpi_accounts = ApproveChecked {
        to: ctx.accounts.maker_atk_account.to_account_info(),
        delegate: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
        mint: ctx.accounts.maker_atk_mint.to_account_info(), // Add missing structure field
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    if let Err(e) = token::approve_checked(cpi_ctx, amount_atk, 0) {
        msg!("Error approving ATK tokens: {:?}", e);
        return Err(e);
    }

    // Store details in escrow
    let escrow = &mut ctx.accounts.escrow_account;
    escrow.amount_atk = amount_atk;
    escrow.amount_btk = amount_btk;
    escrow.mint_btk = mint_btk;
    escrow.maker_btk_token_account = maker_btk_token_account.to_account_info().key();

    msg!("Offer made successfully.");

    Ok(())
}*/
