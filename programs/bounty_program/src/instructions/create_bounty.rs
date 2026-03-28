use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(bounty_id: u64)]
pub struct CreateBounty<'info> {
    #[account(
        init,
        payer = creator,
        space = Bounty::LEN,
        seeds = [
            BOUNTY_SEED,
            creator.key().as_ref(),
            &bounty_id.to_le_bytes()
        ],
        bump
    )]
    pub bounty: Account<'info, Bounty>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateBounty>,
    bounty_id: u64,
    reward: u64,
) -> Result<()> {
    let bounty: &mut Account<'_, Bounty> = &mut ctx.accounts.bounty;
    bounty.creator = ctx.accounts.creator.key();
    bounty.bounty_id = bounty_id;
    bounty.reward = reward;
    bounty.status = BountyStatus::Open;
    bounty.bump = ctx.bumps.bounty;

    Ok(())
} 