use std::num::NonZeroU32;

use sanctum_spl_stake_pool_core::{
    self as stake_pool_sdk, DEPOSIT_STAKE_IX_IS_SIGNER, DEPOSIT_STAKE_IX_IS_WRITER,
};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use tsify_next::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    err::no_valid_pda,
    find_validator_stake_account_pda_internal, find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle, B58PK,
};

use super::Instruction;

#[wasm_bindgen]
#[derive(Default)]
pub struct DepositStakeIxKeysHandle(stake_pool_sdk::DepositStakeIxKeysOwned);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct DepositStakeIxUserAddrs {
    pub program: B58PK,
    pub stake_pool: B58PK,
    pub deposit_stake: B58PK,
    pub validator_vote: B58PK,
    pub pool_tokens_to: B58PK,
    pub referral_pool_tokens: B58PK,
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
    let withdraw_auth = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;
    let validator_stake = find_validator_stake_account_pda_internal(
        &program.0,
        &validator_vote.0,
        &stake_pool.0,
        validator_stake_seed.and_then(NonZeroU32::new),
    )
    .ok_or_else(no_valid_pda)?
    .0;

    let accounts = stake_pool_sdk::DepositStakeIxKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool_handle.0)
        .with_consts()
        .with_stake_pool(stake_pool.0)
        .with_deposit_auth(stake_pool_handle.0.stake_deposit_authority)
        .with_withdraw_auth(withdraw_auth)
        .with_deposit_stake(deposit_stake.0)
        .with_validator_stake(validator_stake)
        .with_pool_tokens_to(pool_tokens_to.0)
        .with_referral_pool_tokens(referral_pool_tokens.0);

    Ok(Instruction {
        data: ByteBuf::from(stake_pool_sdk::DepositStakeIxData::new().to_buf()),
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
