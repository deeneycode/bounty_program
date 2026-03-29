use anchor_lang::prelude::*;

#[account]
pub struct Bounty {
    pub creator: Pubkey,
    pub claimant: Pubkey,
    pub bounty_id: u64,
    pub reward: u64,
    pub status: BountyStatus,
    pub mint: Pubkey,  // spl token mint for the reward
    pub vault: Pubkey, // vault token account (PDA controlled)
    pub bump: u8,
}

impl Bounty {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 32 + 32 + 1; // discriminator + creator + bounty_id + reward + status + mint + vault + bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BountyStatus {
    Open,
    Claimed,
    Closed,
}
