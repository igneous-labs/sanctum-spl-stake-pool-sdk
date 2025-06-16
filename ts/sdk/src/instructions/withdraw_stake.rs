use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use tsify_next::Tsify;

use sanctum_spl_stake_pool_core as stake_pool_sdk;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    err::no_valid_pda,
    find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle, B58PK,
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
    pub program: B58PK,
    pub stake_pool: B58PK,
    pub stake_to_split: B58PK,
    pub stake_to_receive: B58PK,
    pub user_stake_auth: B58PK,
    pub user_transfer_auth: B58PK,
    pub pool_tokens_from: B58PK,
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
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;

    let accounts = WithdrawStakeIxPrefixKeysHandle(
        stake_pool_sdk::WithdrawStakeIxKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool_handle.0)
            .with_stake_pool(stake_pool.0)
            .with_withdraw_auth(withdraw_authority)
            .with_user_transfer_auth(user_transfer_auth.0)
            .with_pool_tokens_from(pool_tokens_from.0)
            .with_stake_to_split(stake_to_split.0)
            .with_stake_to_receive(stake_to_receive.0)
            .with_user_stake_auth(user_stake_auth.0)
            .with_consts(),
    )
    .to_account_metas();

    let data = stake_pool_sdk::WithdrawStakeIxData::new(args.pool_tokens_in);

    Ok(Instruction {
        data: ByteBuf::from(data.to_buf()),
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
