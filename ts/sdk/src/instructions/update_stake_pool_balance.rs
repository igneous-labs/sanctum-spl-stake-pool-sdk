use sanctum_spl_stake_pool_core::{
    self as stake_pool_sdk, UPDATE_STAKE_POOL_BALANCE_IX_IS_SIGNER,
    UPDATE_STAKE_POOL_BALANCE_IX_IS_WRITER,
};
use serde_bytes::ByteBuf;
use tsify_next::declare;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    err::no_valid_pda,
    find_withdraw_auth_pda_internal,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle,
};

use super::{Instruction, ProgramAndStakePoolUserAddrs};

#[wasm_bindgen]
#[derive(Default)]
pub struct UpdateStakePoolBalanceIxKeysHandle(stake_pool_sdk::UpdateStakePoolBalanceIxKeysOwned);

#[declare]
pub type UpdateStakePoolBalanceIxUserAddrs = ProgramAndStakePoolUserAddrs;

/// @throws if
/// - invalid pubkey was provided
/// - PDAs can't be found
#[wasm_bindgen(js_name = updateStakePoolBalanceIxFromStakePool)]
pub fn update_stake_pool_balance_ix_from_stake_pool(
    UpdateStakePoolBalanceIxUserAddrs {
        program,
        stake_pool,
    }: UpdateStakePoolBalanceIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
) -> Result<Instruction, JsError> {
    let withdraw_authority = find_withdraw_auth_pda_internal(&program.0, &stake_pool.0)
        .ok_or_else(no_valid_pda)?
        .0;

    let accounts = stake_pool_sdk::UpdateStakePoolBalanceIxKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool_handle.0)
        .with_stake_pool(stake_pool.0)
        .with_withdraw_auth(withdraw_authority);

    Ok(Instruction {
        data: ByteBuf::from(stake_pool_sdk::UpdateStakePoolBalanceIxData::new().to_buf()),
        accounts: Box::new(UpdateStakePoolBalanceIxKeysHandle(accounts).to_account_metas()),
        program_address: program,
    })
}

impl UpdateStakePoolBalanceIxKeysHandle {
    fn to_account_metas(
        &self,
    ) -> [AccountMeta; stake_pool_sdk::UPDATE_STAKE_POOL_BALANCE_IX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &UPDATE_STAKE_POOL_BALANCE_IX_IS_SIGNER.0,
            &UPDATE_STAKE_POOL_BALANCE_IX_IS_WRITER.0,
        )
    }
}
