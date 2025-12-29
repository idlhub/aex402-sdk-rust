//! Instruction builders

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::constants::{disc, PROGRAM_ID, TOKEN_PROGRAM_ID, TwapWindow};

// ============================================================================
// Helper Functions
// ============================================================================

fn write_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}

fn write_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn write_i64(buf: &mut Vec<u8>, v: i64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

// ============================================================================
// Pool Creation
// ============================================================================

pub fn create_pool(
    pool: &Pubkey,
    mint0: &Pubkey,
    mint1: &Pubkey,
    authority: &Pubkey,
    amp: u64,
    bump: u8,
) -> Instruction {
    let mut data = Vec::with_capacity(17);
    write_u64(&mut data, disc::CREATEPOOL);
    write_u64(&mut data, amp);
    write_u8(&mut data, bump);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*mint0, false),
            AccountMeta::new_readonly(*mint1, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    }
}

pub fn init_t0_vault(
    pool: &Pubkey,
    vault: &Pubkey,
    authority: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: disc::INITT0V.to_le_bytes().to_vec(),
    }
}

pub fn init_t1_vault(
    pool: &Pubkey,
    vault: &Pubkey,
    authority: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: disc::INITT1V.to_le_bytes().to_vec(),
    }
}

pub fn init_lp_mint(
    pool: &Pubkey,
    lp_mint: &Pubkey,
    authority: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*lp_mint, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: disc::INITLPM.to_le_bytes().to_vec(),
    }
}

// ============================================================================
// Swaps
// ============================================================================

pub fn swap(
    pool: &Pubkey,
    vault0: &Pubkey,
    vault1: &Pubkey,
    user_token0: &Pubkey,
    user_token1: &Pubkey,
    user: &Pubkey,
    from: u8,
    to: u8,
    amount_in: u64,
    min_out: u64,
    deadline: i64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(34);
    write_u64(&mut data, disc::SWAP);
    write_u8(&mut data, from);
    write_u8(&mut data, to);
    write_u64(&mut data, amount_in);
    write_u64(&mut data, min_out);
    write_i64(&mut data, deadline);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*vault0, false),
            AccountMeta::new(*vault1, false),
            AccountMeta::new(*user_token0, false),
            AccountMeta::new(*user_token1, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

pub fn swap_t0_t1(
    pool: &Pubkey,
    vault0: &Pubkey,
    vault1: &Pubkey,
    user_token0: &Pubkey,
    user_token1: &Pubkey,
    user: &Pubkey,
    amount_in: u64,
    min_out: u64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(24);
    write_u64(&mut data, disc::SWAPT0T1);
    write_u64(&mut data, amount_in);
    write_u64(&mut data, min_out);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*vault0, false),
            AccountMeta::new(*vault1, false),
            AccountMeta::new(*user_token0, false),
            AccountMeta::new(*user_token1, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

pub fn swap_t1_t0(
    pool: &Pubkey,
    vault0: &Pubkey,
    vault1: &Pubkey,
    user_token0: &Pubkey,
    user_token1: &Pubkey,
    user: &Pubkey,
    amount_in: u64,
    min_out: u64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(24);
    write_u64(&mut data, disc::SWAPT1T0);
    write_u64(&mut data, amount_in);
    write_u64(&mut data, min_out);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*vault0, false),
            AccountMeta::new(*vault1, false),
            AccountMeta::new(*user_token0, false),
            AccountMeta::new(*user_token1, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

// ============================================================================
// Liquidity
// ============================================================================

pub fn add_liquidity(
    pool: &Pubkey,
    vault0: &Pubkey,
    vault1: &Pubkey,
    lp_mint: &Pubkey,
    user_token0: &Pubkey,
    user_token1: &Pubkey,
    user_lp: &Pubkey,
    user: &Pubkey,
    amount0: u64,
    amount1: u64,
    min_lp: u64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(32);
    write_u64(&mut data, disc::ADDLIQ);
    write_u64(&mut data, amount0);
    write_u64(&mut data, amount1);
    write_u64(&mut data, min_lp);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*vault0, false),
            AccountMeta::new(*vault1, false),
            AccountMeta::new(*lp_mint, false),
            AccountMeta::new(*user_token0, false),
            AccountMeta::new(*user_token1, false),
            AccountMeta::new(*user_lp, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

pub fn remove_liquidity(
    pool: &Pubkey,
    vault0: &Pubkey,
    vault1: &Pubkey,
    lp_mint: &Pubkey,
    user_token0: &Pubkey,
    user_token1: &Pubkey,
    user_lp: &Pubkey,
    user: &Pubkey,
    lp_amount: u64,
    min0: u64,
    min1: u64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(32);
    write_u64(&mut data, disc::REMLIQ);
    write_u64(&mut data, lp_amount);
    write_u64(&mut data, min0);
    write_u64(&mut data, min1);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*vault0, false),
            AccountMeta::new(*vault1, false),
            AccountMeta::new(*lp_mint, false),
            AccountMeta::new(*user_token0, false),
            AccountMeta::new(*user_token1, false),
            AccountMeta::new(*user_lp, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

// ============================================================================
// Admin
// ============================================================================

pub fn set_pause(pool: &Pubkey, authority: &Pubkey, paused: bool) -> Instruction {
    let mut data = Vec::with_capacity(9);
    write_u64(&mut data, disc::SETPAUSE);
    write_u8(&mut data, if paused { 1 } else { 0 });

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

pub fn update_fee(pool: &Pubkey, authority: &Pubkey, fee_bps: u64) -> Instruction {
    let mut data = Vec::with_capacity(16);
    write_u64(&mut data, disc::UPDFEE);
    write_u64(&mut data, fee_bps);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

pub fn commit_amp(pool: &Pubkey, authority: &Pubkey, target_amp: u64) -> Instruction {
    let mut data = Vec::with_capacity(16);
    write_u64(&mut data, disc::COMMITAMP);
    write_u64(&mut data, target_amp);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

pub fn ramp_amp(pool: &Pubkey, authority: &Pubkey, target_amp: u64, duration: i64) -> Instruction {
    let mut data = Vec::with_capacity(24);
    write_u64(&mut data, disc::RAMPAMP);
    write_u64(&mut data, target_amp);
    write_i64(&mut data, duration);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data,
    }
}

pub fn stop_ramp(pool: &Pubkey, authority: &Pubkey) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data: disc::STOPRAMP.to_le_bytes().to_vec(),
    }
}

pub fn init_auth_transfer(
    pool: &Pubkey,
    authority: &Pubkey,
    new_authority: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(*new_authority, false),
        ],
        data: disc::INITAUTH.to_le_bytes().to_vec(),
    }
}

pub fn complete_auth_transfer(pool: &Pubkey, new_authority: &Pubkey) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*new_authority, true),
        ],
        data: disc::COMPLAUTH.to_le_bytes().to_vec(),
    }
}

pub fn cancel_auth_transfer(pool: &Pubkey, authority: &Pubkey) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*authority, true),
        ],
        data: disc::CANCELAUTH.to_le_bytes().to_vec(),
    }
}

// ============================================================================
// Farming
// ============================================================================

pub fn stake_lp(
    user_position: &Pubkey,
    farm: &Pubkey,
    user_lp: &Pubkey,
    lp_vault: &Pubkey,
    user: &Pubkey,
    amount: u64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(16);
    write_u64(&mut data, disc::STAKELP);
    write_u64(&mut data, amount);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*user_position, false),
            AccountMeta::new(*farm, false),
            AccountMeta::new(*user_lp, false),
            AccountMeta::new(*lp_vault, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

pub fn unstake_lp(
    user_position: &Pubkey,
    farm: &Pubkey,
    user_lp: &Pubkey,
    lp_vault: &Pubkey,
    user: &Pubkey,
    amount: u64,
    token_program: Option<&Pubkey>,
) -> Instruction {
    let mut data = Vec::with_capacity(16);
    write_u64(&mut data, disc::UNSTAKELP);
    write_u64(&mut data, amount);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*user_position, false),
            AccountMeta::new(*farm, false),
            AccountMeta::new(*user_lp, false),
            AccountMeta::new(*lp_vault, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data,
    }
}

pub fn claim_farm(
    user_position: &Pubkey,
    farm: &Pubkey,
    pool: &Pubkey,
    reward_vault: &Pubkey,
    user_reward: &Pubkey,
    user: &Pubkey,
    token_program: Option<&Pubkey>,
) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*user_position, false),
            AccountMeta::new(*farm, false),
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*reward_vault, false),
            AccountMeta::new(*user_reward, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program.unwrap_or(&TOKEN_PROGRAM_ID), false),
        ],
        data: disc::CLAIMFARM.to_le_bytes().to_vec(),
    }
}

// ============================================================================
// Oracle
// ============================================================================

pub fn get_twap(pool: &Pubkey, window: TwapWindow) -> Instruction {
    let mut data = Vec::with_capacity(9);
    write_u64(&mut data, disc::GETTWAP);
    write_u8(&mut data, window as u8);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*pool, false),
        ],
        data,
    }
}
