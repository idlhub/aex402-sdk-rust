//! Error types

use thiserror::Error;

/// AeX402 error codes
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AeX402Error {
    #[error("Pool is paused")]
    Paused = 6000,

    #[error("Invalid amplification coefficient")]
    InvalidAmp = 6001,

    #[error("Math overflow")]
    MathOverflow = 6002,

    #[error("Zero amount")]
    ZeroAmount = 6003,

    #[error("Slippage exceeded")]
    SlippageExceeded = 6004,

    #[error("Invalid invariant or PDA mismatch")]
    InvalidInvariant = 6005,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity = 6006,

    #[error("Vault mismatch")]
    VaultMismatch = 6007,

    #[error("Expired or ended")]
    Expired = 6008,

    #[error("Already initialized")]
    AlreadyInitialized = 6009,

    #[error("Unauthorized")]
    Unauthorized = 6010,

    #[error("Ramp constraint violated")]
    RampConstraint = 6011,

    #[error("Tokens are locked")]
    Locked = 6012,

    #[error("Farming error")]
    FarmingError = 6013,

    #[error("Invalid account owner")]
    InvalidOwner = 6014,

    #[error("Invalid account discriminator")]
    InvalidDiscriminator = 6015,

    #[error("CPI call failed")]
    CpiFailed = 6016,
}

impl From<u32> for AeX402Error {
    fn from(code: u32) -> Self {
        match code {
            6000 => Self::Paused,
            6001 => Self::InvalidAmp,
            6002 => Self::MathOverflow,
            6003 => Self::ZeroAmount,
            6004 => Self::SlippageExceeded,
            6005 => Self::InvalidInvariant,
            6006 => Self::InsufficientLiquidity,
            6007 => Self::VaultMismatch,
            6008 => Self::Expired,
            6009 => Self::AlreadyInitialized,
            6010 => Self::Unauthorized,
            6011 => Self::RampConstraint,
            6012 => Self::Locked,
            6013 => Self::FarmingError,
            6014 => Self::InvalidOwner,
            6015 => Self::InvalidDiscriminator,
            6016 => Self::CpiFailed,
            _ => Self::MathOverflow, // fallback
        }
    }
}
