//! Program constants and discriminators

use solana_program::pubkey::Pubkey;

/// Program ID
pub const PROGRAM_ID: Pubkey = solana_program::pubkey!("3AMM53MsJZy2Jvf7PeHHga3bsGjWV4TSaYz29WUtcdje");

/// Token Program ID
pub const TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Token-2022 Program ID
pub const TOKEN_2022_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

// Pool parameters
pub const MIN_AMP: u64 = 1;
pub const MAX_AMP: u64 = 100_000;
pub const DEFAULT_FEE_BPS: u64 = 30;
pub const ADMIN_FEE_PCT: u64 = 50;
pub const MIN_SWAP: u64 = 100_000;
pub const MIN_DEPOSIT: u64 = 100_000_000;
pub const NEWTON_ITERATIONS: u8 = 255;
pub const RAMP_MIN_DURATION: i64 = 86_400; // 1 day
pub const COMMIT_DELAY: i64 = 3_600;       // 1 hour
pub const MIGRATION_FEE_BPS: u64 = 1337;   // 0.1337%
pub const MAX_TOKENS: usize = 8;
pub const POOL_SIZE: usize = 1024;
pub const NPOOL_SIZE: usize = 2048;
pub const OHLCV_24H: usize = 24;
pub const OHLCV_7D: usize = 7;

/// Instruction discriminators (little-endian u64)
pub mod disc {
    // Pool creation
    pub const CREATEPOOL: u64 = 0xf2b9e4d1c8a7e3f9;
    pub const CREATEPN: u64 = 0x27c933bce5c77c1b;
    pub const INITT0V: u64 = 0x5e8c3b0d0f3e4a9f;
    pub const INITT1V: u64 = 0x7a4e9f1c3b2d5e8a;
    pub const INITLPM: u64 = 0xf4d1e9a3c5b8e7f2;

    // Swaps
    pub const SWAP: u64 = 0x82c69e91e17587c8;
    pub const SWAPT0T1: u64 = 0x642af2b7e0f14e2a;
    pub const SWAPT1T0: u64 = 0x3a0e131bac75c4c8;
    pub const SWAPN: u64 = 0xf1a8e3c7b2d9e5f8;
    pub const MIGT0T1: u64 = 0xd2e4f1a8c3b7e9d5;
    pub const MIGT1T0: u64 = 0x1888779426393db8;

    // Liquidity
    pub const ADDLIQ: u64 = 0xa2e7c4f8b3d1e5a9;
    pub const ADDLIQ1: u64 = 0x51c98b4e3c2e12e6;
    pub const ADDLIQN: u64 = 0xe3f7a2c8d1b9e4f6;
    pub const REMLIQ: u64 = 0x2e54bc2c75c9f902;
    pub const REMLIQN: u64 = 0xb3f8e2a5c7d9e1b4;

    // Admin
    pub const SETPAUSE: u64 = 0xe075762b7e0d6ec9;
    pub const UPDFEE: u64 = 0x8f3a2e5b7c9d1f4a;
    pub const WDRAWFEE: u64 = 0xf9e5d3a2c8b1e7f8;
    pub const COMMITAMP: u64 = 0xc1d9e3f7a5b8e2c4;
    pub const RAMPAMP: u64 = 0x9a1c5e3f7b2d8e6a;
    pub const STOPRAMP: u64 = 0x3c9427bb15a21053;
    pub const INITAUTH: u64 = 0xf5e2a7c9d3b1e8f4;
    pub const COMPLAUTH: u64 = 0xf6e8d2a4c7b9e1f5;
    pub const CANCELAUTH: u64 = 0xf7e3a9c1d5b2e8f6;

    // Farming
    pub const CREATEFARM: u64 = 0x6d7b0c8e2f1a3d5c;
    pub const STAKELP: u64 = 0xf8d4e1a7c3b9e2f7;
    pub const UNSTAKELP: u64 = 0x4166bf654e34f8bc;
    pub const CLAIMFARM: u64 = 0x075762b7e0d6ec9b;
    pub const LOCKLP: u64 = 0xfefb83015f028cec;
    pub const CLAIMULP: u64 = 0xca8593f45ce88b1e;

    // Lottery
    pub const ENTERLOT: u64 = 0xe795383a4eef48fc;
    pub const DRAWLOT: u64 = 0x1361225a4d7cbc11;
    pub const CLAIMLOT: u64 = 0x7e7b5e3f15f93cf4;

    // Registry
    pub const INITREG: u64 = 0xa1b2c3d4e5f60718;
    pub const REGPOOL: u64 = 0xb2c3d4e5f6071829;
    pub const UNREGPOOL: u64 = 0xc3d4e5f607182930;
    pub const INITREGA: u64 = 0xd4e5f60718293041;
    pub const COMPLREGA: u64 = 0xe5f6071829304152;
    pub const CANCELREGA: u64 = 0xf607182930415263;

    // Oracle
    pub const GETTWAP: u64 = 0x7477617067657401;

    // Transfer Hook
    pub const TH_EXEC: u64 = 0x1a66fb4bc5652569;
    pub const TH_INIT: u64 = 0xebeb58a7310d222b;
}

/// Account discriminators
pub mod account_disc {
    pub const POOL: [u8; 8] = *b"POOLSWAP";
    pub const NPOOL: [u8; 8] = *b"NPOOLSWA";
    pub const FARM: [u8; 8] = *b"FARMSWAP";
    pub const UFARM: [u8; 8] = *b"UFARMSWA";
    pub const LOTTERY: [u8; 8] = *b"LOTTERY!";
    pub const LOTENTRY: [u8; 8] = *b"LOTENTRY";
    pub const REGISTRY: [u8; 8] = *b"REGISTRY";
}

/// TWAP window options
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwapWindow {
    Hour1 = 0,
    Hour4 = 1,
    Hour24 = 2,
    Day7 = 3,
}
