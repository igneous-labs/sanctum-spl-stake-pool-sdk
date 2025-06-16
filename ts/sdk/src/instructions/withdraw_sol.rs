use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use tsify_next::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use sanctum_spl_stake_pool_core as stake_pool_sdk;

use crate::{
    err::no_valid_pda,
    find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta, Role},
    StakePoolHandle, B58PK,
};

use super::Instruction;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawSolIxArgs {
    pub pool_tokens_in: u64,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct WithdrawSolIxPrefixKeysHandle(stake_pool_sdk::WithdrawSolIxPrefixKeysOwned);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawSolIxUserAddrs {
    pub program: B58PK,
    pub stake_pool: B58PK,
    pub user_transfer_auth: B58PK,
    pub pool_tokens_from: B58PK,
    pub lamports_to: B58PK,
}

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[wasm_bindgen(js_name = withdrawSolIxFromStakePool)]
pub fn withdraw_sol_ix_from_stake_pool(
    WithdrawSolIxUserAddrs {
        program,
        stake_pool,
        user_transfer_auth,
        pool_tokens_from,
        lamports_to,
    }: WithdrawSolIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
    args: WithdrawSolIxArgs,
) -> Result<Instruction, JsError> {
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;

    let accounts = WithdrawSolIxPrefixKeysHandle(
        stake_pool_sdk::WithdrawSolIxPrefixKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool_handle.0)
            .with_stake_pool(stake_pool.0)
            .with_withdraw_auth(withdraw_authority)
            .with_user_transfer_auth(user_transfer_auth.0)
            .with_pool_tokens_from(pool_tokens_from.0)
            .with_lamports_to(lamports_to.0)
            .with_consts(),
    )
    .to_account_metas();

    let data = stake_pool_sdk::WithdrawSolIxData::new(args.pool_tokens_in);

    Ok(Instruction {
        data: ByteBuf::from(data.to_buf()),
        accounts: accounts
            .into_iter()
            .chain(
                stake_pool_handle
                    .0
                    .sol_withdraw_authority
                    .into_iter()
                    .map(|auth| AccountMeta::new(auth, Role::ReadonlySigner)),
            )
            .collect(),
        program_address: program,
    })
}

impl WithdrawSolIxPrefixKeysHandle {
    fn to_account_metas(&self) -> [AccountMeta; stake_pool_sdk::WITHDRAW_SOL_IX_PREFIX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &stake_pool_sdk::WITHDRAW_SOL_IX_PREFIX_IS_SIGNER.0,
            &stake_pool_sdk::WITHDRAW_SOL_IX_PREFIX_IS_WRITER.0,
        )
    }
}
