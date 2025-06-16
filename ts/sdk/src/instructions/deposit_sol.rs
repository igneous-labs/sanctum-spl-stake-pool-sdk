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
pub struct DepositSolIxArgs {
    pub deposit_lamports: u64,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct DepositSolIxPrefixKeysHandle(stake_pool_sdk::DepositSolIxPrefixKeysOwned);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct DepositSolIxUserAddrs {
    pub program: B58PK,
    pub stake_pool: B58PK,
    pub referrer_fee: B58PK,
    pub from_user_lamports: B58PK,
    pub dest_user_pool: B58PK,
}

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(js_name = depositSolIxFromStakePool)]
pub fn deposit_sol_ix_from_stake_pool(
    DepositSolIxUserAddrs {
        program,
        stake_pool,
        referrer_fee,
        from_user_lamports,
        dest_user_pool,
    }: DepositSolIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
    args: DepositSolIxArgs,
) -> Result<Instruction, JsError> {
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;

    let accounts = DepositSolIxPrefixKeysHandle(
        stake_pool_sdk::DepositSolIxPrefixKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool_handle.0)
            .with_stake_pool(stake_pool.0)
            .with_withdraw_auth(withdraw_authority)
            .with_referrer_fee(referrer_fee.0)
            .with_from_user_lamports(from_user_lamports.0)
            .with_dest_user_pool(dest_user_pool.0)
            .with_consts(),
    )
    .to_account_metas();

    let data = stake_pool_sdk::DepositSolIxData::new(args.deposit_lamports);

    Ok(Instruction {
        data: ByteBuf::from(data.to_buf()),
        accounts: accounts
            .into_iter()
            .chain(
                stake_pool_handle
                    .0
                    .sol_deposit_authority
                    .into_iter()
                    .map(|auth| AccountMeta::new(auth, Role::ReadonlySigner)),
            )
            .collect(),
        program_address: program,
    })
}

impl DepositSolIxPrefixKeysHandle {
    fn to_account_metas(&self) -> [AccountMeta; stake_pool_sdk::DEPOSIT_SOL_IX_PREFIX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &stake_pool_sdk::DEPOSIT_SOL_IX_PREFIX_IS_SIGNER.0,
            &stake_pool_sdk::DEPOSIT_SOL_IX_PREFIX_IS_WRITER.0,
        )
    }
}
