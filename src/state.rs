//! Account state structures

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::constants::{account_disc, OHLCV_24H, OHLCV_7D, MAX_TOKENS};

/// Delta-encoded OHLCV candle (12 bytes)
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct Candle {
    pub open: u32,      // Base price (scaled 1e6)
    pub high_d: u16,    // High delta
    pub low_d: u16,     // Low delta
    pub close_d: i16,   // Close delta
    pub volume: u16,    // Volume in 1e9 units
}

impl Candle {
    pub fn high(&self) -> u32 {
        self.open.saturating_add(self.high_d as u32)
    }

    pub fn low(&self) -> u32 {
        self.open.saturating_sub(self.low_d as u32)
    }

    pub fn close(&self) -> i32 {
        self.open as i32 + self.close_d as i32
    }
}

/// 2-token Pool state (1024 bytes)
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub disc: [u8; 8],
    pub bump: u8,
    pub paused: u8,
    pub _padding: [u8; 6],
    pub authority: Pubkey,
    pub pending_auth: Pubkey,
    pub auth_time: i64,
    pub mint0: Pubkey,
    pub mint1: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub lp_mint: Pubkey,
    pub bal0: u64,
    pub bal1: u64,
    pub lp_supply: u64,
    pub amp: u64,
    pub target_amp: u64,
    pub ramp_start: i64,
    pub ramp_end: i64,
    pub pending_amp: u64,
    pub commit_time: i64,
    pub fee_bps: u64,
    pub admin_fee0: u64,
    pub admin_fee1: u64,
    pub total_swaps: u64,
    pub total_volume: u64,
    pub last_slot: u64,
    pub hourly_idx: u8,
    pub daily_idx: u8,
    pub _padding2: [u8; 6],
    pub hourly_candles: [Candle; OHLCV_24H],
    pub daily_candles: [Candle; OHLCV_7D],
    pub trade_count: u32,
    pub trade_sum: u64,
    pub max_price: u32,
    pub min_price: u32,
    pub bloom: [u8; 128],
}

impl Pool {
    pub fn is_valid(&self) -> bool {
        self.disc == account_disc::POOL
    }

    pub fn is_paused(&self) -> bool {
        self.paused != 0
    }

    /// Get current effective amp (handles ramping)
    pub fn get_amp(&self, now: i64) -> u64 {
        if now >= self.ramp_end || self.ramp_end == self.ramp_start {
            return self.target_amp;
        }
        if now <= self.ramp_start {
            return self.amp;
        }

        let elapsed = now - self.ramp_start;
        let duration = self.ramp_end - self.ramp_start;

        if self.target_amp > self.amp {
            let diff = self.target_amp - self.amp;
            self.amp + (diff * elapsed as u64) / duration as u64
        } else {
            let diff = self.amp - self.target_amp;
            self.amp - (diff * elapsed as u64) / duration as u64
        }
    }
}

/// N-token Pool state (2048 bytes)
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct NPool {
    pub disc: [u8; 8],
    pub bump: u8,
    pub paused: u8,
    pub n_tokens: u8,
    pub _padding: [u8; 5],
    pub authority: Pubkey,
    pub pending_auth: Pubkey,
    pub auth_time: i64,
    pub mints: [Pubkey; MAX_TOKENS],
    pub vaults: [Pubkey; MAX_TOKENS],
    pub lp_mint: Pubkey,
    pub balances: [u64; MAX_TOKENS],
    pub lp_supply: u64,
    pub amp: u64,
    pub target_amp: u64,
    pub ramp_start: i64,
    pub ramp_end: i64,
    pub pending_amp: u64,
    pub commit_time: i64,
    pub fee_bps: u64,
    pub admin_fees: [u64; MAX_TOKENS],
    pub total_swaps: u64,
    pub total_volume: u64,
}

impl NPool {
    pub fn is_valid(&self) -> bool {
        self.disc == account_disc::NPOOL
    }

    pub fn is_paused(&self) -> bool {
        self.paused != 0
    }
}

/// Farm state
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Farm {
    pub disc: [u8; 8],
    pub bump: u8,
    pub _padding: [u8; 7],
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub lp_vault: Pubkey,
    pub reward_rate: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub last_update: i64,
    pub acc_reward: u128,  // scaled 1e12
    pub total_staked: u64,
}

impl Farm {
    pub fn is_valid(&self) -> bool {
        self.disc == account_disc::FARM
    }
}

/// User farm position
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct UserFarm {
    pub disc: [u8; 8],
    pub bump: u8,
    pub _padding: [u8; 7],
    pub owner: Pubkey,
    pub farm: Pubkey,
    pub staked: u64,
    pub reward_debt: u128,
    pub locked_amount: u64,
    pub unlock_time: i64,
}

impl UserFarm {
    pub fn is_valid(&self) -> bool {
        self.disc == account_disc::UFARM
    }
}

/// Lottery state
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Lottery {
    pub disc: [u8; 8],
    pub bump: u8,
    pub drawn: u8,
    pub _padding: [u8; 6],
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub lp_vault: Pubkey,
    pub ticket_price: u64,
    pub total_tickets: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub winner: Pubkey,
    pub winning_ticket: u64,
}

impl Lottery {
    pub fn is_valid(&self) -> bool {
        self.disc == account_disc::LOTTERY
    }

    pub fn is_drawn(&self) -> bool {
        self.drawn != 0
    }
}

/// Lottery entry
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct LotteryEntry {
    pub disc: [u8; 8],
    pub bump: u8,
    pub claimed: u8,
    pub _padding: [u8; 6],
    pub owner: Pubkey,
    pub lottery: Pubkey,
    pub ticket_start: u64,
    pub ticket_count: u64,
}

impl LotteryEntry {
    pub fn is_valid(&self) -> bool {
        self.disc == account_disc::LOTENTRY
    }

    pub fn is_winner(&self, winning_ticket: u64) -> bool {
        winning_ticket >= self.ticket_start 
            && winning_ticket < self.ticket_start + self.ticket_count
    }
}

/// TWAP result decoded from return value
#[derive(Debug, Clone, Copy)]
pub struct TwapResult {
    pub price: u32,       // Scaled 1e6
    pub samples: u16,     // Number of candles used
    pub confidence: u16,  // 0-10000 (0-100%)
}

impl TwapResult {
    pub fn decode(encoded: u64) -> Self {
        Self {
            price: (encoded & 0xFFFFFFFF) as u32,
            samples: ((encoded >> 32) & 0xFFFF) as u16,
            confidence: ((encoded >> 48) & 0xFFFF) as u16,
        }
    }

    pub fn price_f64(&self) -> f64 {
        self.price as f64 / 1e6
    }

    pub fn confidence_pct(&self) -> f64 {
        self.confidence as f64 / 100.0
    }
}
