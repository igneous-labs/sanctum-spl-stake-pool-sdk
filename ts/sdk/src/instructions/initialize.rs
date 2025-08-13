use sanctum_spl_stake_pool_core::Fee;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use tsify_next::Tsify;

use sanctum_spl_stake_pool_core as stake_pool_sdk;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    err::no_valid_pda,
    find_deposit_auth_pda_internal, find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta, Role},
    B58PK,
};

use super::Instruction;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct InitializeIxArgs {
    pub fee: Fee,
    pub withdrawal_fee: Fee,
    pub deposit_fee: Fee,
    pub referral_fee: u8,
    pub max_validators: u32,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct InitializeIxKeysHandle(stake_pool_sdk::InitializeIxPrefixKeysOwned);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct InitializeIxUserAddrs {
    pub program: B58PK,
    pub stake_pool: B58PK,
    pub manager: B58PK,
    pub manager_fee: B58PK,
    pub staker: B58PK,
    pub validator_list: B58PK,
    pub reserve: B58PK,
    pub pool_mint: B58PK,
    pub pool_token_program: B58PK,

    #[tsify(optional)]
    pub deposit_authority: Option<B58PK>,
}

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(js_name = initializeIx)]
pub fn initialize_ix(
    InitializeIxUserAddrs {
        program,
        stake_pool,
        manager,
        manager_fee,
        staker,
        validator_list,
        reserve,
        pool_mint,
        pool_token_program,
        deposit_authority,
    }: InitializeIxUserAddrs,
    args: InitializeIxArgs,
) -> Result<Instruction, JsError> {
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;

    let deposit_authority = match deposit_authority {
        Some(s) => s.0,
        None => {
            find_deposit_auth_pda_internal(&program.0, &stake_pool.0)
                .ok_or_else(no_valid_pda)?
                .0
        }
    };

    let accounts = InitializeIxKeysHandle(
        stake_pool_sdk::InitializeIxPrefixKeysOwned::default()
            .with_stake_pool(stake_pool.0)
            .with_withdraw_auth(withdraw_authority)
            .with_validator_list(validator_list.0)
            .with_reserve(reserve.0)
            .with_pool_mint(pool_mint.0)
            .with_manager_fee(manager_fee.0)
            .with_manager(manager.0)
            .with_staker(staker.0)
            .with_pool_token_prog(pool_token_program.0),
    )
    .to_account_metas();

    let data = stake_pool_sdk::InitializeIxData::new(
        args.fee,
        args.withdrawal_fee,
        args.deposit_fee,
        args.referral_fee,
        args.max_validators,
    );

    Ok(Instruction {
        data: ByteBuf::from(data.to_buf()),
        accounts: accounts
            .into_iter()
            .chain(std::iter::once(AccountMeta::new(
                deposit_authority,
                Role::Readonly,
            )))
            .collect(),
        program_address: program,
    })
}

impl InitializeIxKeysHandle {
    fn to_account_metas(&self) -> [AccountMeta; stake_pool_sdk::INITIALIZE_IX_PREFIX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &stake_pool_sdk::INITIALIZE_IX_PREFIX_IS_SIGNER.0,
            &stake_pool_sdk::INITIALIZE_IX_PREFIX_IS_WRITER.0,
        )
    }
}
