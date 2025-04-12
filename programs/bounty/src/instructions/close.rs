use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        TokenAccount,
        TokenInterface,
        CloseAccount,
        close_account,
        TransferChecked,
        transfer_checked
    }
};

use crate::state::*;

#[derive(Accounts)]
pub struct Close <'info>{
    #[account(mut)]
    maker: Signer <'info>,
    mint: InterfaceAccount <'info, Mint>,
    #[account(
        mut,
        close = maker,
        has_one = mint,
        seeds = [b"bounty", maker.key().as_ref(), bounty.seed.to_le_bytes().as_ref()],
        bump = bounty.bump,
    )]
    bounty: Account <'info, Bounty>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bounty
    )]
    spl_vault: InterfaceAccount <'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = maker
    )]
    maker_ata: InterfaceAccount <'info, TokenAccount>,
    associated_token_program: Program <'info, AssociatedToken>,
    token_program: Interface <'info, TokenInterface>,
    system_program: Program <'info, System>
}

pub fn close(ctx: Context<Close>) -> Result<()> {
    let signer_seeds:[&[&[u8]]; 1] = [&[
        b"bounty",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.bounty.seed.to_le_bytes()[..],
        &[ctx.accounts.bounty.bump]
    ]];

    let cpi_transfer = ctx.accounts.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.spl_vault.to_account_info(),
        to: ctx.accounts.maker_ata.to_account_info(),
        authority: ctx.accounts.bounty.to_account_info(),
        mint: ctx.accounts.mint.to_account_info()
    };
    
    let cpi_ctx = CpiContext::new_with_signer(cpi_transfer, cpi_accounts, &signer_seeds);
    
    transfer_checked(cpi_ctx, ctx.accounts.spl_vault.amount, ctx.accounts.mint.decimals)?;

    let signer_seeds_close:[&[&[u8]]; 1] = [&[
        b"bounty",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.bounty.seed.to_le_bytes()[..],
        &[ctx.accounts.bounty.bump]
    ]];

    let close_accounts = CloseAccount {
        account: ctx.accounts.spl_vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.bounty.to_account_info(),
    };

    let cpi_close_program = ctx.accounts.token_program.to_account_info();

    let cpi_close_tx = CpiContext::new_with_signer(cpi_close_program, close_accounts, &signer_seeds_close);

    close_account(cpi_close_tx)?;

    Ok(())
}
