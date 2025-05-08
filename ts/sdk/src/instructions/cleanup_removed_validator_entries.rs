use crate::{
    conv::pubkey_from_js,
    utils::{keys_signer_writer_to_account_metas, AccountMeta},
    StakePoolHandle,
};
use sanctum_spl_stake_pool_core::{
    self as stake_pool_sdk, CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_IS_SIGNER,
    CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_IS_WRITER,
};
use tsify_next::declare;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use super::{Instruction, ProgramAndStakePoolUserAddrs};

#[wasm_bindgen]
#[derive(Default)]
pub struct CleanupRemovedValidatorEntriesIxKeysHandle(
    stake_pool_sdk::CleanupRemovedValidatorEntriesIxKeysOwned,
);

#[declare]
pub type CleanupRemovedValidatorEntriesIxUserAddrs = ProgramAndStakePoolUserAddrs;

/// @throws if
/// - invalid pubkey was provided
#[wasm_bindgen(js_name = cleanupRemovedValidatorEntriesIxFromStakePool)]
pub fn cleanup_removed_validator_entries_ix_from_stake_pool(
    CleanupRemovedValidatorEntriesIxUserAddrs {
        program,
        stake_pool,
    }: CleanupRemovedValidatorEntriesIxUserAddrs,
    stake_pool_handle: &StakePoolHandle,
) -> Result<Instruction, JsError> {
    let accounts = stake_pool_sdk::CleanupRemovedValidatorEntriesIxKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool_handle.0)
        .with_stake_pool(pubkey_from_js(&stake_pool)?);

    Ok(Instruction {
        data: Box::new(stake_pool_sdk::CleanupRemovedValidatorEntriesIxData::new().to_buf()),
        accounts: Box::new(CleanupRemovedValidatorEntriesIxKeysHandle(accounts).to_account_metas()),
        program_address: program,
    })
}

impl CleanupRemovedValidatorEntriesIxKeysHandle {
    fn to_account_metas(
        &self,
    ) -> [AccountMeta; stake_pool_sdk::CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCS_LEN] {
        keys_signer_writer_to_account_metas(
            &self.0.as_borrowed().0,
            &CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_IS_SIGNER.0,
            &CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_IS_WRITER.0,
        )
    }
}
