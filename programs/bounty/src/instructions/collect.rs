use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint, TokenAccount,
        TokenInterface,
        CloseAccount,
        close_account,
        TransferChecked,
        transfer_checked
    }
};

use crate::error::ErrorCode;
use crate::state::*;

#[derive(Accounts)]
pub struct Collect <'info> {
    #[account(mut)]
    taker: Signer <'info>,
    #[account(mut)]
    maker: SystemAccount<'info>,
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
        init_if_needed,
        payer = taker,
        associated_token::mint = mint,
        associated_token::authority = taker
    )]
    taker_ata: InterfaceAccount <'info, TokenAccount>,
    associated_token_program: Program <'info, AssociatedToken>,
    token_program: Interface <'info, TokenInterface>,
    system_program: Program <'info, System>
}

pub fn collect(ctx: Context<Collect>, pull_request_status: u16) -> Result<()> {
    require!(
        pull_request_status == 204,
        ErrorCode::PullRequestNotMerged
    );

    let signer_seeds:[&[&[u8]]; 1] = [&[
        b"bounty",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.bounty.seed.to_le_bytes()[..],
        &[ctx.accounts.bounty.bump]
    ]];

    let cpi_transfer = ctx.accounts.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.spl_vault.to_account_info(),
        to: ctx.accounts.taker_ata.to_account_info(),
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
