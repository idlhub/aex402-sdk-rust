//! StableSwap math for off-chain simulation

use crate::constants::NEWTON_ITERATIONS;

/// Calculate invariant D for 2-token pool using Newton's method
pub fn calc_d(x: u64, y: u64, amp: u64) -> Option<u64> {
    let s = x.checked_add(y)?;
    if s == 0 {
        return Some(0);
    }

    let mut d = s;
    let ann = amp.checked_mul(4)?; // A * n^n where n=2

    for _ in 0..NEWTON_ITERATIONS {
        // d_p = d^3 / (4 * x * y)
        let d_p = (d as u128)
            .checked_mul(d as u128)?
            .checked_div(x.checked_mul(2)? as u128)?
            .checked_mul(d as u128)?
            .checked_div(y.checked_mul(2)? as u128)?;

        let d_prev = d;

        // d = (ann * s + d_p * 2) * d / ((ann - 1) * d + 3 * d_p)
        let num = (ann as u128)
            .checked_mul(s as u128)?
            .checked_add(d_p.checked_mul(2)?)?
            .checked_mul(d as u128)?;
        
        let denom = (ann.checked_sub(1)? as u128)
            .checked_mul(d as u128)?
            .checked_add(d_p.checked_mul(3)?)?;

        d = (num / denom) as u64;

        // Check convergence
        let diff = if d > d_prev { d - d_prev } else { d_prev - d };
        if diff <= 1 {
            return Some(d);
        }
    }

    None // Failed to converge
}

/// Calculate output amount y given input x for swap
pub fn calc_y(x_new: u64, d: u64, amp: u64) -> Option<u64> {
    let ann = amp.checked_mul(4)?;

    // c = d^3 / (4 * x_new * ann)
    let c = (d as u128)
        .checked_mul(d as u128)?
        .checked_div(x_new.checked_mul(2)? as u128)?
        .checked_mul(d as u128)?
        .checked_div(ann.checked_mul(2)? as u128)?;

    // b = x_new + d / ann
    let b = x_new.checked_add(d / ann)?;

    let mut y = d;

    for _ in 0..NEWTON_ITERATIONS {
        let y_prev = y;

        // y = (y^2 + c) / (2y + b - d)
        let num = (y as u128)
            .checked_mul(y as u128)?
            .checked_add(c)?;
        
        let denom = y
            .checked_mul(2)?
            .checked_add(b)?
            .checked_sub(d)?;

        y = (num / denom as u128) as u64;

        // Check convergence
        let diff = if y > y_prev { y - y_prev } else { y_prev - y };
        if diff <= 1 {
            return Some(y);
        }
    }

    None
}

/// Simulate a swap and return output amount
pub fn simulate_swap(
    bal_in: u64,
    bal_out: u64,
    amount_in: u64,
    amp: u64,
    fee_bps: u64,
) -> Option<u64> {
    let d = calc_d(bal_in, bal_out, amp)?;
    let new_bal_in = bal_in.checked_add(amount_in)?;
    let new_bal_out = calc_y(new_bal_in, d, amp)?;
    let mut amount_out = bal_out.checked_sub(new_bal_out)?;

    // Apply fee
    let fee = amount_out.checked_mul(fee_bps)? / 10000;
    amount_out = amount_out.checked_sub(fee)?;

    Some(amount_out)
}

/// Calculate LP tokens for deposit (2-token pool)
pub fn calc_lp_tokens(
    amt0: u64,
    amt1: u64,
    bal0: u64,
    bal1: u64,
    lp_supply: u64,
    amp: u64,
) -> Option<u64> {
    if lp_supply == 0 {
        // Initial deposit: LP = sqrt(amt0 * amt1)
        let product = (amt0 as u128).checked_mul(amt1 as u128)?;
        return Some(isqrt(product) as u64);
    }

    let d0 = calc_d(bal0, bal1, amp)?;
    let d1 = calc_d(bal0.checked_add(amt0)?, bal1.checked_add(amt1)?, amp)?;

    if d0 == 0 {
        return None;
    }

    // LP tokens = lp_supply * (d1 - d0) / d0
    let lp = (lp_supply as u128)
        .checked_mul(d1.checked_sub(d0)? as u128)?
        / d0 as u128;

    Some(lp as u64)
}

/// Calculate tokens received for LP burn
pub fn calc_withdraw(
    lp_amount: u64,
    bal0: u64,
    bal1: u64,
    lp_supply: u64,
) -> Option<(u64, u64)> {
    if lp_supply == 0 {
        return None;
    }

    let amount0 = (bal0 as u128)
        .checked_mul(lp_amount as u128)?
        / lp_supply as u128;
    
    let amount1 = (bal1 as u128)
        .checked_mul(lp_amount as u128)?
        / lp_supply as u128;

    Some((amount0 as u64, amount1 as u64))
}

/// Calculate current amp during ramping
pub fn get_current_amp(
    amp: u64,
    target_amp: u64,
    ramp_start: i64,
    ramp_end: i64,
    now: i64,
) -> u64 {
    if now >= ramp_end || ramp_end == ramp_start {
        return target_amp;
    }

    if now <= ramp_start {
        return amp;
    }

    let elapsed = (now - ramp_start) as u64;
    let duration = (ramp_end - ramp_start) as u64;

    if target_amp > amp {
        let diff = target_amp - amp;
        amp + (diff * elapsed) / duration
    } else {
        let diff = amp - target_amp;
        amp - (diff * elapsed) / duration
    }
}

/// Calculate price impact for a swap
pub fn calc_price_impact(
    bal_in: u64,
    bal_out: u64,
    amount_in: u64,
    amp: u64,
    fee_bps: u64,
) -> Option<f64> {
    let amount_out = simulate_swap(bal_in, bal_out, amount_in, amp, fee_bps)?;
    
    // Price impact = 1 - (amount_out / amount_in)
    let ratio = (amount_out as f64) / (amount_in as f64);
    Some(1.0 - ratio)
}

/// Calculate minimum output with slippage tolerance
pub fn calc_min_output(expected: u64, slippage_bps: u64) -> u64 {
    let slippage = expected.saturating_mul(slippage_bps) / 10000;
    expected.saturating_sub(slippage)
}

/// Calculate virtual price (LP value relative to underlying)
pub fn calc_virtual_price(bal0: u64, bal1: u64, lp_supply: u64, amp: u64) -> Option<u128> {
    if lp_supply == 0 {
        return None;
    }

    let d = calc_d(bal0, bal1, amp)?;
    
    // Virtual price = D * 1e18 / lp_supply
    const PRECISION: u128 = 1_000_000_000_000_000_000; // 1e18
    Some((d as u128).checked_mul(PRECISION)? / lp_supply as u128)
}

/// Integer square root using Newton's method
fn isqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }

    let mut x = n;
    let mut y = (x + 1) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_d() {
        let d = calc_d(1_000_000_000_000, 1_000_000_000_000, 1000).unwrap();
        assert!(d > 0);
        assert!(d >= 2_000_000_000_000); // D >= sum of balances
    }

    #[test]
    fn test_simulate_swap() {
        let bal = 1_000_000_000_000u64;
        let out = simulate_swap(bal, bal, 10_000_000_000, 1000, 30).unwrap();
        
        // Output should be slightly less than input (due to fee + curve)
        assert!(out < 10_000_000_000);
        assert!(out > 9_900_000_000); // Not too much slippage
    }

    #[test]
    fn test_price_impact() {
        let bal = 1_000_000_000_000u64;
        let impact = calc_price_impact(bal, bal, 10_000_000_000, 1000, 30).unwrap();
        
        // Price impact should be small for balanced pool
        assert!(impact < 0.01); // Less than 1%
        assert!(impact > 0.0);  // But non-zero
    }
}
