use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bounty {
    pub bump: u8,
    pub seed: u64,
    pub maker: Pubkey,
    #[max_len(200)]
    pub repository_url: String,
    pub amount: u64,
    pub taker: Pubkey,
    pub pull_request_number: u16,
    pub mint: Pubkey,
}
