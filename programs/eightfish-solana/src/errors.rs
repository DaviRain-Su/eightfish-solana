use anchor_lang::prelude::*;

#[error_code]
pub enum EightFishError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Cannot get the bump.")]
    CannotGetBump,
    #[msg("payload too large")]
    PayloadTooLarge,
}
