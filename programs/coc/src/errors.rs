use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // access control errors
    #[msg("Controller has already been initialized.")]
    AlreadyInitialized,
    #[msg("Admin must be the signer.")]
    AdminMustSign,

    #[msg("Role name cannot be empty.")]
    InvalidRoleName,
    #[msg("User is already assigned with the role")]
    UserAlreadyAssigned,

    // industry errors
    #[msg("Bond amount is invalid")]
    InvalidBondAmount,
    #[msg("Company name is invalid")]
    InvalidCompanyName,
    #[msg("Company name is invalid")]
    InvalidCompany,
    #[msg("Registration number is invalid")]
    InvalidRegistrationNumber,
    #[msg("System program is invalid")]
    InvalidSystemProgram,

    // report emission errors
    #[msg("Industry not active")]
    IndustryNotActive,
    #[msg("Emission amount cant be 0")]
    InvalidEmissionAmount,
    #[msg("token account owner is invalid")]
    InvalidTokenAccountOwner,

    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Insufficient permissions for this action")]
    InsufficientPermissions,
    #[msg("Token is not active")]
    TokenNotActive,
    #[msg("Auction is not active")]
    AuctionNotActive,
    #[msg("Auction has expired")]
    AuctionExpired,
    #[msg("Insufficient tokens available in auction")]
    InsufficientTokensAvailable,
    #[msg("Math operation overflow")]
    MathOverflow,
}
