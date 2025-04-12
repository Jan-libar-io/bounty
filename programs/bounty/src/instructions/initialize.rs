use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked
    }
};

use crate::error::ErrorCode;
use crate::state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize <'info> {
    #[account(mut)]
    maker: Signer <'info>,
    mint: InterfaceAccount <'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = maker
    )]
    maker_ata: InterfaceAccount <'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = 8 + Bounty::INIT_SPACE,
        seeds = [b"bounty", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    bounty: Account <'info, Bounty>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint,
        associated_token::authority = bounty
    )]
    spl_vault: InterfaceAccount <'info, TokenAccount>,
    associated_token_program: Program <'info, AssociatedToken>,
    token_program: Interface <'info, TokenInterface>,
    system_program: Program <'info, System>
}

pub fn initialize(ctx: Context<Initialize>, seed: u64, repository_url: String, amount: u64) -> Result<()> {
    require!(repository_url.len() < 201, ErrorCode::URLTooLong);

    ctx.accounts.bounty.set_inner(Bounty {
        seed,
        maker: ctx.accounts.maker.key(),
        repository_url,
        amount,
        bump: ctx.bumps.bounty,
        taker: Pubkey::default(),
        pull_request_number: 0,
        mint: ctx.accounts.mint.key(),
    });

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.maker_ata.to_account_info(),
        to: ctx.accounts.spl_vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, ctx.accounts.bounty.amount, ctx.accounts.mint.decimals)?;

    Ok(())
}
