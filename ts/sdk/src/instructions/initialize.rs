use sanctum_spl_stake_pool_core::Fee;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use sanctum_spl_stake_pool_core as stake_pool_sdk;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    conv::{pubkey_from_js, pubkey_to_js},
    err::no_valid_pda,
    find_deposit_auth_pda_internal, find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta, Role},
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
    pub program: Box<str>,
    pub stake_pool: Box<str>,
    pub manager: Box<str>,
    pub manager_fee: Box<str>,
    pub staker: Box<str>,
    pub validator_list: Box<str>,
    pub reserve: Box<str>,
    pub pool_mint: Box<str>,
    pub pool_token_program: Box<str>,

    #[tsify(optional)]
    pub deposit_authority: Option<Box<str>>,
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
    let program_addr = pubkey_from_js(&program)?;
    let stake_pool_addr = pubkey_from_js(&stake_pool)?;
    let manager_addr = pubkey_from_js(&manager)?;
    let staker_addr = pubkey_from_js(&staker)?;
    let validator_list_addr = pubkey_from_js(&validator_list)?;
    let reserve_addr = pubkey_from_js(&reserve)?;
    let pool_mint_addr = pubkey_from_js(&pool_mint)?;
    let pool_token_program_addr = pubkey_from_js(&pool_token_program)?;
    let manager_fee_addr = pubkey_from_js(&manager_fee)?;
    let withdraw_authority = find_withdraw_auth_pda_internal(&program_addr, &stake_pool_addr)
        .ok_or_else(no_valid_pda)?
        .0;

    let deposit_authority = match deposit_authority {
        Some(s) => pubkey_from_js(&s)?,
        None => {
            find_deposit_auth_pda_internal(&program_addr, &stake_pool_addr)
                .ok_or_else(no_valid_pda)?
                .0
        }
    };

    let accounts = InitializeIxKeysHandle(
        stake_pool_sdk::InitializeIxPrefixKeysOwned::default()
            .with_stake_pool(stake_pool_addr)
            .with_withdraw_auth(withdraw_authority)
            .with_validator_list(validator_list_addr)
            .with_reserve(reserve_addr)
            .with_pool_mint(pool_mint_addr)
            .with_manager_fee(manager_fee_addr)
            .with_manager(manager_addr)
            .with_staker(staker_addr)
            .with_pool_token_prog(pool_token_program_addr),
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
        data: data.to_buf().into(),
        accounts: accounts
            .into_iter()
            .chain(std::iter::once(AccountMeta::new(
                pubkey_to_js(&deposit_authority),
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
