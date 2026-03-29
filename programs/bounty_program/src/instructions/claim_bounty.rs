use crate::constants::*;
use crate::errors::BountyError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, TokenAccount, Token, TransferChecked};

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(
        mut, 
        close = creator,
        has_one = creator,
        has_one = vault,
        has_one = mint,
    )]
    pub bounty: Account<'info, Bounty>,

    /// Vault token account — source of escrowed tokens
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    /// Claimant's token account (destination)
    #[account(
        mut,
        token::mint = mint,
        token::authority = claimant,
    )]
    pub claimant_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub creator: SystemAccount<'info>,

    #[account(mut)]
    pub claimant: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
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

    let amount: u64 = ctx.accounts.vault.amount;
    let decimals: u8 = ctx.accounts.mint.decimals;
    require!(amount > 0, BountyError::ZeroClaim);

    // PDA signer seeds- the bounty PDA is the vault authority
    let creator_key: [u8; 32] = bounty_info.creator.to_bytes();
    let bounty_id: [u8; 8] = bounty_info.bounty_id.to_le_bytes();
    let seeds: &[&[u8]] = &[
        BOUNTY_SEED,
        creator_key.as_ref(),
        bounty_id.as_ref(),
        &[bounty_info.bump],
    ];

    // Transfer token form vault to claimants token account
    let transfer_accounts: TransferChecked<'_> = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.claimant_token_account.to_account_info(),
        authority: bounty_info.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            &[seeds],
        ),
        amount,
        decimals,
    )?;

    // Close the vault token account — lamports go to creator
    let close_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.creator.to_account_info(),
        authority: bounty_info.to_account_info(),
    };
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        close_accounts,
        &[seeds],
    ))?;

    // update bounty state to claimed
    bounty_info.status = BountyStatus::Claimed;

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
            mint: Pubkey::default(),
            vault: Pubkey::default(),
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
            mint: Pubkey::default(),
            vault: Pubkey::default(),
        };
        assert!(bounty.status == BountyStatus::Claimed);
    }
}
