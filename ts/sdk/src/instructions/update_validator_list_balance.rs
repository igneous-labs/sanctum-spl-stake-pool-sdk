use crate::{
    err::{no_valid_pda, validator_idx_oob},
    find_pda, find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta, Role},
    StakePoolHandle, ValidatorListHandle,
};
use core::convert::TryFrom;
use sanctum_spl_stake_pool_core::{
    self as stake_pool_sdk, UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_IS_SIGNER,
    UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_IS_WRITER,
};
use serde::{Deserialize, Serialize};

use tsify_next::{declare, Tsify};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use super::{Instruction, ProgramAndStakePoolUserAddrs};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct UpdateValidatorListBalanceIxArgs {
    pub start_index: usize,
    pub no_merge: bool,
    pub count: usize,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct UpdateValidatorListBalanceIxPrefixKeysHandle(
    stake_pool_sdk::UpdateValidatorListBalanceIxPrefixKeysOwned,
);

#[declare]
pub type UpdateValidatorListBalanceIxUserAddrs = ProgramAndStakePoolUserAddrs;

/// @throws if
/// - start index or count is out of bounds
/// - invalid pubkey was provided
/// - PDAs can't be found
#[wasm_bindgen(js_name = updateValidatorListBalanceIxFromStakePool)]
pub fn update_validator_list_balance_ix_from_stake_pool(
    UpdateValidatorListBalanceIxUserAddrs {
        program,
        stake_pool,
    }: UpdateValidatorListBalanceIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
    validator_list_handle: &ValidatorListHandle,
    args: UpdateValidatorListBalanceIxArgs,
) -> Result<Instruction, JsError> {
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;
    let validator_list = validator_list_handle.0.as_borrowed();

    if args.start_index >= validator_list.validators.len() {
        return Err(validator_idx_oob());
    }

    if args.start_index + args.count > validator_list.validators.len() {
        return Err(validator_idx_oob());
    }

    let accounts = UpdateValidatorListBalanceIxPrefixKeysHandle(
        stake_pool_sdk::UpdateValidatorListBalanceIxPrefixKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool_handle.0)
            .with_stake_pool(stake_pool.0)
            .with_withdraw_auth(withdraw_authority)
            .with_consts(),
    )
    .to_account_metas();

    let vsa_tsa_pairs = validator_list
        .account_pair_seeds_itr(&stake_pool.0)
        .skip(args.start_index)
        .take(args.count)
        .flat_map(|((v1, v2, v3), (t1, t2, t3, t4))| {
            [
                find_pda(&[v1.as_slice(), v2.as_slice(), v3.as_slice()], &program.0),
                find_pda(&[t1, t2, t3, &t4], &program.0),
            ]
            .into_iter()
            .map(|pda_opt| {
                pda_opt.map_or_else(
                    || Err(no_valid_pda()),
                    |(pda, _bump)| Ok(AccountMeta::new(pda, Role::Writable)),
                )
            })
        });

    let data = stake_pool_sdk::UpdateValidatorListBalanceIxData::new(
        u32::try_from(args.start_index)?,
        args.no_merge,
    )
    .to_buf();

    let accounts: Result<Box<[AccountMeta]>, JsError> =
        accounts.into_iter().map(Ok).chain(vsa_tsa_pairs).collect();

    Ok(Instruction {
        data: data.as_slice().into(),
        accounts: accounts?,
        program_address: program,
    })
}

impl UpdateValidatorListBalanceIxPrefixKeysHandle {
    fn to_account_metas(
        &self,
    ) -> [AccountMeta; stake_pool_sdk::UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_IS_SIGNER.0,
            &UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_IS_WRITER.0,
        )
    }
}
