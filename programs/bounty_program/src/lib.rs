use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;

use crate::instructions::*;


declare_id!("C2yr5N11oGn4k2gvKwwvvA7nffZE8vYuP94NLch1YbA4");

#[program]
pub mod bounty {
    use super::*;
    
    pub fn create_bounty(
        ctx: Context<CreateBounty>,
        bounty_id: u64,
        reward: u64,
    ) -> Result<()> {
        instructions::create_bounty::handler(ctx, bounty_id, reward)
    }

    pub fn fund_bounty(
        ctx: Context<FundBounty>,
        amount: u64,
    ) -> Result<()> {
        instructions::fund_bounty::handler(ctx, amount)
    }

    pub fn claim_bounty(
        ctx: Context<ClaimBounty>,
    ) -> Result<()> {
        instructions::claim_bounty::handler(ctx)
    }

    pub fn close_bounty(
        ctx: Context<CloseBounty>,
    ) -> Result<()> {
        instructions::close_bounty::handler(ctx)
    }

}


