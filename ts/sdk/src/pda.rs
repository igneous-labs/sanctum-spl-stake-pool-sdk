use std::{iter::once, num::NonZeroU32};

use ed25519_compact::{PublicKey, Signature};
use sanctum_spl_stake_pool_core::{
    deposit_auth_seeds, ephemeral_stake_seeds, transient_stake_seeds, validator_stake_seeds,
    withdraw_auth_seeds,
};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::{err::no_valid_pda, B58PK};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
pub struct FoundPda(pub B58PK, pub u8);

/// @throws
/// - if pubkey params are not valid pubkey strings
/// - if no valid PDA was found
#[wasm_bindgen(js_name = findWithdrawAuthPda)]
pub fn find_withdraw_auth_pda(
    program_id: B58PK,
    stake_pool_addr: B58PK,
) -> Result<FoundPda, JsError> {
    find_withdraw_auth_pda_internal(&program_id.0, &stake_pool_addr.0)
        .map(|(pk, bump)| FoundPda(B58PK::new(pk), bump))
        .ok_or_else(no_valid_pda)
}

/// @throws
/// - if pubkey params are not valid pubkey strings
/// - if no valid PDA was found
#[wasm_bindgen(js_name = findDepositAuthPda)]
pub fn find_deposit_auth_pda(
    program_id: B58PK,
    stake_pool_addr: B58PK,
) -> Result<FoundPda, JsError> {
    find_deposit_auth_pda_internal(&program_id.0, &stake_pool_addr.0)
        .map(|(pk, bump)| FoundPda(B58PK::new(pk), bump))
        .ok_or_else(no_valid_pda)
}

/// @throws
/// - if pubkey params are not valid pubkey strings
/// - if no valid PDA was found
#[wasm_bindgen(js_name = findValidatorStakeAccountPda)]
pub fn find_validator_stake_account_pda(
    program_id: B58PK,
    vote_account_addr: B58PK,
    stake_pool_addr: B58PK,
    seed: Option<u32>,
) -> Result<FoundPda, JsError> {
    let seed = seed.and_then(NonZeroU32::new);

    find_validator_stake_account_pda_internal(
        &program_id.0,
        &vote_account_addr.0,
        &stake_pool_addr.0,
        seed,
    )
    .map(|(pk, bump)| FoundPda(B58PK::new(pk), bump))
    .ok_or_else(no_valid_pda)
}

/// @throws
/// - if pubkey params are not valid pubkey strings
/// - if no valid PDA was found
#[wasm_bindgen(js_name = findTransientStakeAccountPda)]
pub fn find_transient_stake_account_pda(
    program_id: B58PK,
    vote_account_addr: B58PK,
    stake_pool_addr: B58PK,
    seed: u64,
) -> Result<FoundPda, JsError> {
    find_transient_stake_account_pda_internal(
        &program_id.0,
        &vote_account_addr.0,
        &stake_pool_addr.0,
        seed,
    )
    .map(|(pk, bump)| FoundPda(B58PK::new(pk), bump))
    .ok_or_else(no_valid_pda)
}

/// @throws
/// - if pubkey params are not valid pubkey strings
/// - if no valid PDA was found
#[wasm_bindgen(js_name = findEphemeralStakeAccountPda)]
pub fn find_ephemeral_stake_account_pda(
    program_id: B58PK,
    stake_pool_addr: B58PK,
) -> Result<FoundPda, JsError> {
    find_ephemeral_stake_account_pda_internal(&program_id.0, &stake_pool_addr.0)
        .map(|(pk, bump)| FoundPda(B58PK::new(pk), bump))
        .ok_or_else(no_valid_pda)
}

pub fn find_withdraw_auth_pda_internal(
    program_id: &[u8; 32],
    stake_pool_addr: &[u8; 32],
) -> Option<([u8; 32], u8)> {
    let (s1, s2) = withdraw_auth_seeds(stake_pool_addr);
    find_pda(&[s1.as_slice(), s2.as_slice()], program_id)
}

pub fn find_deposit_auth_pda_internal(
    program_id: &[u8; 32],
    stake_pool_addr: &[u8; 32],
) -> Option<([u8; 32], u8)> {
    let (s1, s2) = deposit_auth_seeds(stake_pool_addr);
    find_pda(&[s1.as_slice(), s2.as_slice()], program_id)
}

pub(crate) fn find_validator_stake_account_pda_internal(
    program_id: &[u8; 32],
    vote_account_addr: &[u8; 32],
    stake_pool_addr: &[u8; 32],
    seed: Option<NonZeroU32>,
) -> Option<([u8; 32], u8)> {
    let (s1, s2, s3) = validator_stake_seeds(vote_account_addr, stake_pool_addr, seed);
    find_pda(&[s1.as_slice(), s2.as_slice(), s3.as_slice()], program_id)
}

pub(crate) fn find_transient_stake_account_pda_internal(
    program_id: &[u8; 32],
    vote_account_addr: &[u8; 32],
    stake_pool_addr: &[u8; 32],
    seed: u64,
) -> Option<([u8; 32], u8)> {
    let (s1, s2, s3, s4) = transient_stake_seeds(vote_account_addr, stake_pool_addr, seed);
    find_pda(
        &[s1.as_slice(), s2.as_slice(), s3.as_slice(), s4.as_slice()],
        program_id,
    )
}

pub(crate) fn find_ephemeral_stake_account_pda_internal(
    program_id: &[u8; 32],
    stake_pool_addr: &[u8; 32],
) -> Option<([u8; 32], u8)> {
    let (s1, s2, s3) = ephemeral_stake_seeds(stake_pool_addr);
    find_pda(&[s1.as_slice(), s2.as_slice(), s3.as_slice()], program_id)
}

/// maximum length of derived `Pubkey` seed
const MAX_SEED_LEN: usize = 32;
/// Maximum number of seeds
const MAX_SEEDS: usize = 16;

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

fn create_pda(
    seeds: impl IntoIterator<Item = impl AsRef<[u8]>>,
    program_id: &[u8; 32],
) -> Option<[u8; 32]> {
    let mut seed_len = 0;
    let mut hasher = hmac_sha256::Hash::new();
    seeds.into_iter().try_for_each(|seed| {
        seed_len += 1;
        if seed_len > MAX_SEEDS || seed.as_ref().len() > MAX_SEED_LEN {
            None
        } else {
            hasher.update(seed);
            Some(())
        }
    })?;
    hasher.update(program_id);
    hasher.update(PDA_MARKER);
    let hash = hasher.finalize();
    // ed25519_compact only checks whether pubkey is on curve
    // when attempting to verify a signature so we try to verify a dummy one
    match PublicKey::new(hash).verify_incremental(&Signature::new([0u8; 64])) {
        // point is on curve
        //
        // See impl of verify_incremental():
        // https://github.com/jedisct1/rust-ed25519-compact/blob/00af8ee6778da59f57ecbe799a02ae5eb95495d9/src/ed25519.rs#L210
        Ok(_) | Err(ed25519_compact::Error::WeakPublicKey) => None,
        // point is not on curve
        Err(ed25519_compact::Error::InvalidPublicKey) => Some(hash),
        Err(_) => unreachable!(),
    }
}

pub fn find_pda(seeds: &[&[u8]], program_id: &[u8; 32]) -> Option<([u8; 32], u8)> {
    // Reference: https://github.com/anza-xyz/solana-sdk/blob/4e30766b8d327f0191df6490e48d9ef521956495/pubkey/src/lib.rs#L633
    // if you look at the impl, 0 is not a valid bump seed, only 1-255 are
    (1..=u8::MAX)
        .rev()
        .filter_map(|bump| {
            let bump_arr = [bump];
            let bump_slice = &bump_arr.as_slice();
            create_pda(seeds.iter().chain(once(bump_slice)), program_id).map(|pda| (pda, bump))
        })
        .next()
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use solana_pubkey::Pubkey;

    use super::*;

    proptest! {
        #[test]
        fn check_create_pda_against_solana(
            program_id: [u8; 32],
            // inclusive max range to test out of bounds erroring too
            seeds in proptest::collection::vec(proptest::collection::vec(any::<u8>(), 0..=MAX_SEED_LEN), 0..=MAX_SEEDS)
        ) {
            let slice_vec: Vec<&[u8]> = seeds.iter().map(|v| v.as_slice()).collect();
            let us = create_pda(&slice_vec, &program_id);
            let sol = Pubkey::create_program_address(&slice_vec, &Pubkey::new_from_array(program_id));

            match (us, sol) {
                (Some(us), Ok(sol)) => prop_assert_eq!(us, sol.to_bytes()),
                (None, Err(_)) => (),
                (us, sol) => panic!("us: {:#?}. sol: {:#?}", us, sol),
            }
        }
    }

    proptest! {
        #[test]
        fn check_find_pda_against_solana(
            program_id: [u8; 32],
            // inclusive max range to test out of bounds erroring too
            seeds in proptest::collection::vec(proptest::collection::vec(any::<u8>(), 0..=MAX_SEED_LEN), 0..=MAX_SEEDS)
        ) {
            let slice_vec: Vec<&[u8]> = seeds.iter().map(|v| v.as_slice()).collect();
            let us = find_pda(&slice_vec, &program_id);
            let sol = Pubkey::try_find_program_address(&slice_vec, &Pubkey::new_from_array(program_id));

            match (us, sol) {
                (Some(us), Some(sol)) => {
                    prop_assert_eq!(us.0, sol.0.to_bytes());
                    prop_assert_eq!(us.1, sol.1);
                }
                (None, None) => (),
                (us, sol) => panic!("us: {:#?}. sol: {:#?}", us, sol),
            }
        }
    }
}
