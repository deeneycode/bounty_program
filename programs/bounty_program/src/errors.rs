use anchor_lang::prelude::*;

#[error_code]
pub enum BountyError {
    #[msg("Bounty is already claimed")]
    AlreadyClaimed,

    #[msg("Unauthorized user")]
    Unauthorized,

    #[msg("Invalid State")]
    InvalidState,

    #[msg("Bounty reward must be greater than zero")]
    ZeroClaim,

    #[msg("Bounty is not open")]
    NotOpen,
}
