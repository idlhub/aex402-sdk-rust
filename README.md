# aex402-sdk

Rust SDK for the AeX402 AMM on Solana - Hybrid StableSwap with Virtual Pools.

## Features

- **Instruction builders** for all handlers
- **Account state parsing** with Borsh
- **StableSwap math** for off-chain simulation
- **Type-safe error handling**

## Installation

```toml
[dependencies]
aex402-sdk = "0.1.0"
```

## Quick Start

```rust
use aex402_sdk::{instruction, math, state::Pool, PROGRAM_ID};
use solana_sdk::pubkey::Pubkey;

// Build swap instruction
let ix = instruction::swap_t0_t1(
    &pool,
    &vault0,
    &vault1,
    &user_token0,
    &user_token1,
    &user,
    1_000_000,  // amount_in
    990_000,    // min_out
    None,       // default token program
);

// Simulate swap off-chain
let expected_out = math::simulate_swap(
    bal0,       // balance of input token
    bal1,       // balance of output token
    amount_in,
    amp,
    fee_bps,
).expect("simulation failed");

// Calculate min output with 0.5% slippage
let min_out = math::calc_min_output(expected_out, 50);
```

## Modules

### `instruction`

```rust
// Pool creation
instruction::create_pool(&pool, &mint0, &mint1, &authority, amp, bump)
instruction::init_t0_vault(&pool, &vault, &authority)
instruction::init_t1_vault(&pool, &vault, &authority)
instruction::init_lp_mint(&pool, &lp_mint, &authority)

// Swaps
instruction::swap(&pool, &v0, &v1, &u0, &u1, &user, from, to, amt, min, deadline, None)
instruction::swap_t0_t1(&pool, &v0, &v1, &u0, &u1, &user, amt, min, None)
instruction::swap_t1_t0(&pool, &v0, &v1, &u0, &u1, &user, amt, min, None)

// Liquidity
instruction::add_liquidity(&pool, &v0, &v1, &lp, &u0, &u1, &ulp, &user, a0, a1, min_lp, None)
instruction::remove_liquidity(&pool, &v0, &v1, &lp, &u0, &u1, &ulp, &user, lp_amt, m0, m1, None)

// Admin
instruction::set_pause(&pool, &authority, paused)
instruction::update_fee(&pool, &authority, fee_bps)
instruction::commit_amp(&pool, &authority, target_amp)
instruction::ramp_amp(&pool, &authority, target_amp, duration)
instruction::stop_ramp(&pool, &authority)

// Farming
instruction::stake_lp(&pos, &farm, &ulp, &vault, &user, amount, None)
instruction::unstake_lp(&pos, &farm, &ulp, &vault, &user, amount, None)
instruction::claim_farm(&pos, &farm, &pool, &rv, &ur, &user, None)

// Oracle
instruction::get_twap(&pool, TwapWindow::Hour24)
```

### `state`

```rust
use aex402_sdk::state::{Pool, NPool, Farm, TwapResult};
use borsh::BorshDeserialize;

// Parse pool from account data
let pool = Pool::try_from_slice(&account.data)?;
assert!(pool.is_valid());

// Get current amp during ramping
let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)?
    .as_secs() as i64;
let current_amp = pool.get_amp(now);

// Decode TWAP result
let result = TwapResult::decode(return_value);
println!("Price: {}", result.price_f64());
println!("Confidence: {}%", result.confidence_pct());
```

### `math`

```rust
use aex402_sdk::math;

// Calculate invariant D
let d = math::calc_d(bal0, bal1, amp)?;

// Calculate output for swap
let y = math::calc_y(new_x, d, amp)?;

// Full swap simulation
let out = math::simulate_swap(bal_in, bal_out, amt, amp, fee_bps)?;

// LP token calculation
let lp = math::calc_lp_tokens(amt0, amt1, bal0, bal1, supply, amp)?;

// Withdrawal calculation
let (out0, out1) = math::calc_withdraw(lp_amount, bal0, bal1, supply)?;

// Price impact
let impact = math::calc_price_impact(bal_in, bal_out, amt, amp, fee_bps)?;

// Virtual price
let vp = math::calc_virtual_price(bal0, bal1, supply, amp)?;
```

### `constants`

```rust
use aex402_sdk::{PROGRAM_ID, disc, TwapWindow};

// Program ID
println!("{}", PROGRAM_ID);

// Discriminators
let swap_disc = disc::SWAPT0T1.to_le_bytes();

// TWAP windows
let window = TwapWindow::Hour24;
```

### `error`

```rust
use aex402_sdk::AeX402Error;

// Parse error from code
let err = AeX402Error::from(6004);
println!("{}", err); // "Slippage exceeded"
```

## Testing

```bash
cargo test
```

## License

MIT
