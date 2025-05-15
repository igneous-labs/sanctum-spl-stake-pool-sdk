use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::JsError;

use crate::B58PK;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct Lockup {
    pub unix_timestamp: i64,
    pub epoch: u64,
    pub custodian: B58PK,
}

impl Lockup {
    pub(crate) fn try_to_core(&self) -> Result<sanctum_spl_stake_pool_core::Lockup, JsError> {
        let Self {
            unix_timestamp,
            epoch,
            custodian,
        } = self;
        Ok(sanctum_spl_stake_pool_core::Lockup {
            unix_timestamp: *unix_timestamp,
            epoch: *epoch,
            custodian: custodian.0,
        })
    }

    pub(crate) fn from_core(
        sanctum_spl_stake_pool_core::Lockup {
            unix_timestamp,
            epoch,
            custodian,
        }: &sanctum_spl_stake_pool_core::Lockup,
    ) -> Self {
        Self {
            unix_timestamp: *unix_timestamp,
            epoch: *epoch,
            custodian: B58PK::new(*custodian),
        }
    }
}
