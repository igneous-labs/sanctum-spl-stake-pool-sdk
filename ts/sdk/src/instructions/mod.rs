mod cleanup_removed_validator_entries;
mod deposit_sol;
mod deposit_stake;
mod increase_additional_validator_stake;
mod initialize;
mod update_stake_pool_balance;
mod update_validator_list_balance;
mod withdraw_sol;
mod withdraw_stake;
pub use cleanup_removed_validator_entries::*;
pub use deposit_sol::*;
pub use deposit_stake::*;
pub use increase_additional_validator_stake::*;
pub use initialize::*;
pub use update_stake_pool_balance::*;
pub use update_validator_list_balance::*;
pub use withdraw_sol::*;
pub use withdraw_stake::*;

use crate::{utils::AccountMeta, B58PK};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub data: Box<[u8]>,
    pub accounts: Box<[AccountMeta]>,
    pub program_address: B58PK,
}

/// This user addrs struct is common across multiple instructions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAndStakePoolUserAddrs {
    pub program: B58PK,
    pub stake_pool: B58PK,
}
