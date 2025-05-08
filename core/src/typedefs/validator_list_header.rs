use super::AccountType;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct ValidatorListHeader {
    /// Account type, must be `ValidatorList` currently
    pub account_type: AccountType,

    /// Maximum allowable number of validators
    pub max_validators: u32,
}

impl ValidatorListHeader {
    inherent_borsh_serde!();
}
