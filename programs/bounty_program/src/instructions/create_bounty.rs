use crate::constants::*;
use crate::state::*;
use anchor_lang::prelude::*;

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
    /// CHECK: This account is only used to store the claimant's pubkey, no data is read or written
    pub claimant: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateBounty>, bounty_id: u64, reward: u64) -> Result<()> {
    let bounty: &mut Account<'_, Bounty> = &mut ctx.accounts.bounty;
    bounty.creator = ctx.accounts.creator.key();
    bounty.claimant = ctx.accounts.claimant.key();
    bounty.bounty_id = bounty_id;
    bounty.reward = reward;
    bounty.status = BountyStatus::Open;
    bounty.bump = ctx.bumps.bounty;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounty_state() {
    let bounty = Bounty {
        creator: Pubkey::default(),
        claimant: Pubkey::default(),
        bounty_id: 0,
        reward: 0,
        status: BountyStatus::Open,
        bump: 0,
    };
    assert!(bounty.creator == Pubkey::default());
    assert!(bounty.claimant == Pubkey::default());
    assert!(bounty.bounty_id == 0);
    assert!(bounty.reward == 0);
    assert!(bounty.status == BountyStatus::Open);
    assert!(bounty.bump == 0);
    }
}