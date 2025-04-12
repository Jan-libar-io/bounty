use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::*;

#[derive(Accounts)]
pub struct Reject <'info>{
    #[account(mut)]
    maker: Signer <'info>,
    #[account(
        mut,
        seeds = [b"bounty", maker.key().as_ref(), bounty.seed.to_le_bytes().as_ref()],
        bump = bounty.bump,
    )]
    bounty: Account <'info, Bounty>,
}

pub fn reject(ctx: Context<Reject>, pull_request_status: u16) -> Result<()> {
    require!(pull_request_status != 204, ErrorCode::SubmissionAccepted);

    ctx.accounts.bounty.pull_request_number = 0;
    ctx.accounts.bounty.taker = Pubkey::default();

    Ok(())
}
