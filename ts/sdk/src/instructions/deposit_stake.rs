use std::num::NonZeroU32;

use sanctum_spl_stake_pool_core::{
    self as stake_pool_sdk, DEPOSIT_STAKE_IX_IS_SIGNER, DEPOSIT_STAKE_IX_IS_WRITER,
};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    conv::pubkey_from_js,
    err::no_valid_pda,
    find_validator_stake_account_pda_internal, find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle,
};

use super::Instruction;

#[wasm_bindgen]
#[derive(Default)]
pub struct DepositStakeIxKeysHandle(stake_pool_sdk::DepositStakeIxKeysOwned);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct DepositStakeIxUserAddrs {
    pub program: Box<str>,
    pub stake_pool: Box<str>,
    pub deposit_stake: Box<str>,
    pub validator_vote: Box<str>,
    pub pool_tokens_to: Box<str>,
    pub referral_pool_tokens: Box<str>,
}

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(js_name = depositStakeIxFromStakePool)]
pub fn deposit_stake_ix_from_stake_pool(
    DepositStakeIxUserAddrs {
        program,
        stake_pool,
        deposit_stake,
        validator_vote,
        pool_tokens_to,
        referral_pool_tokens,
    }: DepositStakeIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
    validator_stake_seed: Option<u32>,
) -> Result<Instruction, JsError> {
    let program_addr = pubkey_from_js(&program)?;
    let stake_pool_addr = pubkey_from_js(&stake_pool)?;
    let deposit_stake_addr = pubkey_from_js(&deposit_stake)?;
    let validator_vote_addr = pubkey_from_js(&validator_vote)?;
    let pool_tokens_to_addr = pubkey_from_js(&pool_tokens_to)?;
    let referral_pool_tokens_addr = pubkey_from_js(&referral_pool_tokens)?;
    let withdraw_auth = find_withdraw_auth_pda_internal(&program_addr, &stake_pool_addr)
        .ok_or_else(no_valid_pda)?
        .0;
    let validator_stake = find_validator_stake_account_pda_internal(
        &program_addr,
        &validator_vote_addr,
        &stake_pool_addr,
        validator_stake_seed.and_then(NonZeroU32::new),
    )
    .ok_or_else(no_valid_pda)?
    .0;

    let accounts = stake_pool_sdk::DepositStakeIxKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool_handle.0)
        .with_consts()
        .with_stake_pool(stake_pool_addr)
        .with_deposit_auth(stake_pool_handle.0.stake_deposit_authority)
        .with_withdraw_auth(withdraw_auth)
        .with_deposit_stake(deposit_stake_addr)
        .with_validator_stake(validator_stake)
        .with_pool_tokens_to(pool_tokens_to_addr)
        .with_referral_pool_tokens(referral_pool_tokens_addr);

    Ok(Instruction {
        data: Box::new(stake_pool_sdk::DepositStakeIxData::new().to_buf()),
        accounts: Box::new(DepositStakeIxKeysHandle(accounts).to_account_metas()),
        program_address: program,
    })
}

impl DepositStakeIxKeysHandle {
    fn to_account_metas(&self) -> [AccountMeta; stake_pool_sdk::DEPOSIT_STAKE_IX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &DEPOSIT_STAKE_IX_IS_SIGNER.0,
            &DEPOSIT_STAKE_IX_IS_WRITER.0,
        )
    }
}
