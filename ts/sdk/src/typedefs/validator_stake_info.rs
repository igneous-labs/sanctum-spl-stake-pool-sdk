use std::num::NonZeroU32;

use sanctum_spl_stake_pool_core::StakeStatus;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::JsError;

use crate::B58PK;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorStakeInfo {
    /// Amount of lamports on the validator stake account, including rent
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub active_stake_lamports: u64,

    /// Amount of transient stake delegated to this validator
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub transient_stake_lamports: u64,

    /// Last epoch the active and transient stake lamports fields were updated
    pub last_update_epoch: u64,

    /// Transient account seed suffix, used to derive the transient stake
    /// account address
    pub transient_seed_suffix: u64,

    // NonZeroU32 is not supported by tsify
    /// Validator account seed suffix
    pub validator_seed_suffix: u32,

    /// Status of the validator stake account
    pub status: StakeStatus,

    /// Validator vote account address
    pub vote_account_address: B58PK,
}

impl ValidatorStakeInfo {
    pub(crate) fn try_to_core(
        &self,
    ) -> Result<sanctum_spl_stake_pool_core::ValidatorStakeInfo, JsError> {
        let Self {
            active_stake_lamports,
            transient_stake_lamports,
            last_update_epoch,
            transient_seed_suffix,
            validator_seed_suffix,
            status,
            vote_account_address,
        } = self;
        let mut res = sanctum_spl_stake_pool_core::ValidatorStakeInfo::DEFAULT;
        res.set_active_stake_lamports(*active_stake_lamports);
        res.set_transient_stake_lamports(*transient_stake_lamports);
        res.set_last_update_epoch(*last_update_epoch);
        res.set_transient_seed_suffix(*transient_seed_suffix);
        res.set_validator_seed_suffix(NonZeroU32::new(*validator_seed_suffix));
        res.set_status(*status);
        res.set_vote_account_address(vote_account_address.0);
        Ok(res)
    }

    pub(crate) fn from_core(vsi: &sanctum_spl_stake_pool_core::ValidatorStakeInfo) -> Self {
        Self {
            active_stake_lamports: vsi.active_stake_lamports(),
            transient_stake_lamports: vsi.transient_stake_lamports(),
            last_update_epoch: vsi.last_update_epoch(),
            transient_seed_suffix: vsi.transient_seed_suffix(),
            validator_seed_suffix: vsi.validator_seed_suffix().map_or_else(|| 0, |n| n.get()),
            status: vsi.status(),
            vote_account_address: B58PK::new(*vsi.vote_account_address()),
        }
    }
}
