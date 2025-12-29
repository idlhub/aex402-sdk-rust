//! AeX402 AMM Rust SDK
//!
//! This crate provides types, instructions, and math utilities for
//! interacting with the AeX402 AMM on Solana.
//!
//! # Example
//! ```ignore
//! use aex402_sdk::{instruction, math, state::Pool};
//! use solana_sdk::pubkey::Pubkey;
//!
//! // Build swap instruction
//! let ix = instruction::swap_t0_t1(
//!     &pool_pubkey,
//!     &vault0,
//!     &vault1,
//!     &user_token0,
//!     &user_token1,
//!     &user,
//!     1_000_000,
//!     990_000,
//! );
//!
//! // Simulate swap off-chain
//! let out = math::simulate_swap(bal0, bal1, amount_in, amp, fee_bps);
//! ```

pub mod constants;
pub mod error;
pub mod instruction;
pub mod math;
pub mod state;

pub use constants::*;
pub use error::AeX402Error;

// Backwards compatibility
pub use error::AeX402Error as StableSwapError;
