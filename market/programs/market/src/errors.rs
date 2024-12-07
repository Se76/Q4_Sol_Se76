use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("the string is too big")]
    StringIsTooBig
}