use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the message authority can update this message")]
    Unauthorized,
    #[msg("Message exceeds the maximum allowed length")]
    MessageTooLong,
}
