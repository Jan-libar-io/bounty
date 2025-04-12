pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("DmdMx7mD3Q3MrWGrPLBFZvznbPkna3PE7L5MxHD5Zras");

#[program]
pub mod bounty {
    use super::*;

    pub fn initialize_bounty(ctx: Context<Initialize>, seed: u64, repository_url: String, amount: u64) -> Result<()> {
        initialize::initialize(
            ctx,
            seed,
            repository_url,
            amount
        )
    }

    pub fn submit_solution(ctx: Context<Submit>, pull_request_number: u16) -> Result<()> {
        submit::submit(ctx, pull_request_number)
    }

    pub fn reject_solution(ctx: Context<Reject>, pull_request_status: u16) -> Result<()> {
        reject::reject(ctx, pull_request_status)
    }

    pub fn collect_bounty(ctx: Context<Collect>, pull_request_status: u16) -> Result<()> {
        collect::collect(ctx, pull_request_status)
    }

    pub fn close_bounty(ctx: Context<Close>) -> Result<()> {
        close::close(ctx)
    }
}
