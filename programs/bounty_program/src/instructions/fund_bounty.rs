use crate::errors::BountyError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, TransferChecked};

#[derive(Accounts)]
pub struct FundBounty<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,
    /// Funders token account to transfer reward tokens from
    #[account(
    mut,
    token::mint = mint,
    token::authority = funder,
    )]
    pub funder_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        has_one = mint,
        has_one = vault,
    )]
    pub bounty: Account<'info, Bounty>,

    /// Vault token account (destination for the transfer, owed by bounty PDA)
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<FundBounty>, amount: u64) -> Result<()> {
    require!(
        ctx.accounts.bounty.status == BountyStatus::Open,
        BountyError::NotOpen
    );
    let decimals: u8 = ctx.accounts.mint.decimals;

    let cpi_accounts: TransferChecked<'_> = TransferChecked {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.funder_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.funder.to_account_info(),
    };
    let cpi_ctx: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> =
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    token::transfer_checked(cpi_ctx, amount, decimals)?;

    Ok(())
}
