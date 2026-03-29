use crate::errors::BountyError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, TokenAccount, Token, TransferChecked};
use crate::constants::*;

#[derive(Accounts)]
pub struct CloseBounty<'info> {
    #[account(
        mut, 
        close = creator,
        has_one = creator,
        has_one = mint,
        has_one = vault,
    )]
    pub bounty: Account<'info, Bounty>,

    /// Vault token account token refunded here on cancellation
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    /// Creator's token account — receives refund
    #[account(
        mut,
        token::mint = mint,
        token::authority = creator,
    )]
    pub creator_token_account: Account<'info, TokenAccount>,

    pub creator: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
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

    let refund_amount: u64 = ctx.accounts.vault.amount;
    let decimals: u8 = ctx.accounts.mint.decimals;

    let creator_key: [u8; 32] = bounty.creator.to_bytes();
    let bounty_id: [u8; 8] = bounty.bounty_id.to_le_bytes();
    let bump: u8 = bounty.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        BOUNTY_SEED,
        creator_key.as_ref(),
        bounty_id.as_ref(),
        &[bump],
    ]];

    if refund_amount > 0 {
        let transfer_accounts: TransferChecked<'_> = TransferChecked {
            from:      ctx.accounts.vault.to_account_info(),
            to:        ctx.accounts.creator_token_account.to_account_info(),
            authority: bounty.to_account_info(),
            mint:      ctx.accounts.mint.to_account_info(),
        };
        token::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_accounts,
                signer_seeds,
            ),
            refund_amount,
            decimals,
        )?;   
    }
    
    // Close the vault token account
    let close_accounts = CloseAccount {
        account:     ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.creator.to_account_info(),
        authority:   bounty.to_account_info(),
    };
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        close_accounts,
        signer_seeds,
    ))?;

    bounty.status = BountyStatus::Closed;

    Ok(())
}
