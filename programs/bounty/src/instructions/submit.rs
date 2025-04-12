use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct Submit <'info>{
    #[account(mut)]
    taker: Signer <'info>,
    #[account(
        mut,
        seeds = [b"bounty", bounty.maker.key().as_ref(), bounty.seed.to_le_bytes().as_ref()],
        bump = bounty.bump,
    )]
    bounty: Account <'info, Bounty>,
}

pub fn submit(ctx: Context<Submit>, pull_request_number: u16) -> Result<()> {
    ctx.accounts.bounty.pull_request_number = pull_request_number;
    ctx.accounts.bounty.taker = ctx.accounts.taker.key();

    Ok(())
}
