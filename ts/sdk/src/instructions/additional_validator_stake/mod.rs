use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::B58PK;

mod decrease;
mod increase;

pub use decrease::*;
pub use increase::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalValidatorStakeIxArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub validator_stake_seed: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalValidatorStakeIxUserAddrs {
    pub program: B58PK,
    pub vote_account: B58PK,
    pub stake_pool: B58PK,
}
