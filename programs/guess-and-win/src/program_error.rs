use anchor_lang::prelude::*;

#[error_code]
pub enum OperationError {
    #[msg("Can't access this account, no rights!")]
    NotOwner,
    #[msg("Not enough SOL to pay!")]
    NotEnoughSOL,
}
