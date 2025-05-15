use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use sanctum_spl_stake_pool_core::{self as stake_pool_sdk};

use crate::{
    err::no_valid_pda,
    find_ephemeral_stake_account_pda_internal, find_transient_stake_account_pda_internal,
    find_validator_stake_account_pda_internal, find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle, B58PK,
};

use super::Instruction;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct IncreaseAdditionalValidatorStakeIxArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub validator_stake_seed: Option<u32>,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct IncreaseAdditionalValidatorStakeIxKeysHandle(
    stake_pool_sdk::IncreaseAdditionalValidatorStakeIxKeysOwned,
);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct IncreaseAdditionalValidatorStakeIxUserAddrs {
    pub program: B58PK,
    pub vote_account: B58PK,
    pub stake_pool: B58PK,
}

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[wasm_bindgen(js_name = increaseAdditionalValidatorStakeIxFromStakePool)]
pub fn increase_additional_validator_stake_ix_from_stake_pool(
    IncreaseAdditionalValidatorStakeIxUserAddrs {
        program,
        vote_account,
        stake_pool,
    }: IncreaseAdditionalValidatorStakeIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
    args: IncreaseAdditionalValidatorStakeIxArgs,
) -> Result<Instruction, JsError> {
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;
    let ephemeral_stake_account =
        find_ephemeral_stake_account_pda_internal(&program.0, &stake_pool.0)
            .ok_or_else(no_valid_pda)?
            .0;
    let transient_stake_account = find_transient_stake_account_pda_internal(
        &program.0,
        &vote_account.0,
        &stake_pool.0,
        args.transient_stake_seed,
    )
    .ok_or_else(no_valid_pda)?
    .0;
    let validator_stake_account = find_validator_stake_account_pda_internal(
        &program.0,
        &vote_account.0,
        &stake_pool.0,
        args.validator_stake_seed.and_then(NonZeroU32::new),
    )
    .ok_or_else(no_valid_pda)?
    .0;

    let accounts = stake_pool_sdk::IncreaseAdditionalValidatorStakeIxKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool_handle.0)
        .with_stake_pool(stake_pool.0)
        .with_withdraw_auth(withdraw_authority)
        .with_ephemeral_stake(ephemeral_stake_account)
        .with_transient_stake(transient_stake_account)
        .with_validator_stake(validator_stake_account)
        .with_validator_vote(vote_account.0)
        .with_consts();

    let data = stake_pool_sdk::IncreaseAdditionalValidatorStakeIxData::new(
        args.lamports,
        args.transient_stake_seed,
    )
    .to_buf();

    Ok(Instruction {
        data: data.as_slice().into(),
        accounts: Box::new(
            IncreaseAdditionalValidatorStakeIxKeysHandle(accounts).to_account_metas(),
        ),
        program_address: program,
    })
}

impl IncreaseAdditionalValidatorStakeIxKeysHandle {
    fn to_account_metas(
        &self,
    ) -> [AccountMeta; stake_pool_sdk::INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &stake_pool_sdk::INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_PREFIX_IS_SIGNER.0,
            &stake_pool_sdk::INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_PREFIX_IS_WRITER.0,
        )
    }
}
