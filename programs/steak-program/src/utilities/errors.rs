use anchor_lang::prelude::*;

#[error_code]
pub enum SteakError {
    #[msg("Invalid collection update authority")]
    InvalidCollectionAuthority,
    #[msg("Invalid collection symbol")]
    InvalidCollectionSymbol,
}
