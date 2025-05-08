use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use sanctum_spl_stake_pool_core as stake_pool_sdk;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    conv::pubkey_from_js,
    err::no_valid_pda,
    find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle,
};

use super::Instruction;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawStakeIxArgs {
    pub pool_tokens_in: u64,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct WithdrawStakeIxPrefixKeysHandle(stake_pool_sdk::WithdrawStakeIxKeysOwned);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawStakeIxUserAddrs {
    pub program: Box<str>,
    pub stake_pool: Box<str>,
    pub stake_to_split: Box<str>,
    pub stake_to_receive: Box<str>,
    pub user_stake_auth: Box<str>,
    pub user_transfer_auth: Box<str>,
    pub pool_tokens_from: Box<str>,
}

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[wasm_bindgen(js_name = withdrawStakeIxFromStakePool)]
pub fn withdraw_stake_ix_from_stake_pool(
    WithdrawStakeIxUserAddrs {
        program,
        stake_pool,
        stake_to_split,
        stake_to_receive,
        user_stake_auth,
        user_transfer_auth,
        pool_tokens_from,
    }: WithdrawStakeIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
    args: WithdrawStakeIxArgs,
) -> Result<Instruction, JsError> {
    let program_addr = pubkey_from_js(&program)?;
    let stake_pool_addr = pubkey_from_js(&stake_pool)?;
    let user_transfer_auth_addr = pubkey_from_js(&user_transfer_auth)?;
    let pool_tokens_from_addr = pubkey_from_js(&pool_tokens_from)?;
    let stake_to_split_addr = pubkey_from_js(&stake_to_split)?;
    let stake_to_receive_addr = pubkey_from_js(&stake_to_receive)?;
    let user_stake_auth_addr = pubkey_from_js(&user_stake_auth)?;

    let withdraw_authority = find_withdraw_auth_pda_internal(&program_addr, &stake_pool_addr)
        .ok_or_else(no_valid_pda)?
        .0;

    let accounts = WithdrawStakeIxPrefixKeysHandle(
        stake_pool_sdk::WithdrawStakeIxKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool_handle.0)
            .with_stake_pool(stake_pool_addr)
            .with_withdraw_auth(withdraw_authority)
            .with_user_transfer_auth(user_transfer_auth_addr)
            .with_pool_tokens_from(pool_tokens_from_addr)
            .with_stake_to_split(stake_to_split_addr)
            .with_stake_to_receive(stake_to_receive_addr)
            .with_user_stake_auth(user_stake_auth_addr)
            .with_consts(),
    )
    .to_account_metas();

    let data = stake_pool_sdk::WithdrawStakeIxData::new(args.pool_tokens_in);

    Ok(Instruction {
        data: data.to_buf().into(),
        accounts: Box::new(accounts),
        program_address: program,
    })
}

impl WithdrawStakeIxPrefixKeysHandle {
    fn to_account_metas(&self) -> [AccountMeta; stake_pool_sdk::WITHDRAW_STAKE_IX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &stake_pool_sdk::WITHDRAW_STAKE_IX_PREFIX_IS_SIGNER.0,
            &stake_pool_sdk::WITHDRAW_STAKE_IX_PREFIX_IS_WRITER.0,
        )
    }
}
