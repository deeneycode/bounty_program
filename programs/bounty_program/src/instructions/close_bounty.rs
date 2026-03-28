use crate::errors::BountyError;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseBounty<'info> {
    #[account(
        mut, 
        close = creator,
        has_one = creator,
    )]
    pub bounty: Account<'info, Bounty>,
    #[account(mut)]
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<CloseBounty>) -> Result<()> {
    let bounty: &mut Account<'_, Bounty> = &mut ctx.accounts.bounty;

    require!(
        bounty.status != BountyStatus::Claimed,
        BountyError::AlreadyClaimed
    );
    require!(
        bounty.creator == ctx.accounts.creator.key(),
        BountyError::Unauthorized
    );

    bounty.status = BountyStatus::Closed;

    Ok(())
}
