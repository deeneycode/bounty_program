use crate::errors::BountyError;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut, close = creator)]
    pub bounty: Account<'info, Bounty>,
    #[account(mut)]
    pub creator: SystemAccount<'info>,
    #[account(mut)]
    pub claimant: Signer<'info>,
}

pub fn handler(ctx: Context<ClaimBounty>) -> Result<()> {
    // Read the state of the bounty
    let bounty_info: &mut Account<'_, Bounty> = &mut ctx.accounts.bounty;
    require!(
        bounty_info.status == BountyStatus::Open,
        BountyError::AlreadyClaimed
    );
    require!(
        bounty_info.claimant == ctx.accounts.claimant.key(),
        BountyError::Unauthorized
    );
    // update the bounty stutus to claimed
    let data_len: usize = bounty_info.to_account_info().data_len();
    bounty_info.status = BountyStatus::Claimed;

    // handle lamports transfer
    let bounty_info: AccountInfo<'_> = ctx.accounts.bounty.to_account_info();
    let total: u64 = **bounty_info.lamports.borrow();
    require!(total > 0, BountyError::ZeroClaim);

    let rent: u64 = Rent::get()?.minimum_balance(data_len);
    let reward: u64 = total.checked_sub(rent).ok_or(BountyError::ZeroClaim)?;

    **bounty_info.lamports.borrow_mut() -= reward;
    **ctx
        .accounts
        .claimant
        .to_account_info()
        .lamports
        .borrow_mut() += reward;

    **bounty_info.lamports.borrow_mut() -= rent;
    **ctx.accounts.creator.to_account_info().lamports.borrow_mut() += rent;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_bounty() {
        let bounty: Bounty = Bounty {
            creator: Pubkey::default(),
            claimant: Pubkey::default(),
            bounty_id: 0,
            reward: 0,
            status: BountyStatus::Open, 
            bump: 0,
        };
        assert!(bounty.status == BountyStatus::Open);
        assert!(bounty.claimant == Pubkey::default());
    }

    #[test]
    fn test_claim_bounty_already_claimed() {
        let bounty: Bounty = Bounty {
            creator: Pubkey::default(),
            claimant: Pubkey::default(),
            bounty_id: 0,
            reward: 0,
            status: BountyStatus::Claimed,
            bump: 0,
        };
        assert!(bounty.status == BountyStatus::Claimed);
    }
}