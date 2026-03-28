use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct FundBounty<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,
    #[account(mut)]
    pub bounty: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<FundBounty>,
    amount: u64,
) -> Result<()> {
    let create_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.funder.key(),
        &ctx.accounts.bounty.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &create_instruction,
        &[
            ctx.accounts.funder.to_account_info(),
            ctx.accounts.bounty.to_account_info(),
        ],
    )?;

    Ok(())
}

