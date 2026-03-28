use anchor_lang::prelude::*;

#[account]
pub struct Bounty {
    pub creator: Pubkey,
    pub claimant: Pubkey,
    pub bounty_id: u64,
    pub reward: u64,
    pub status: BountyStatus,
    pub bump: u8,
}

impl Bounty {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1 + 1; // discriminator + creator + bounty_id + reward + status + bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BountyStatus {
    Open,
    Claimed,
    Closed,
}
