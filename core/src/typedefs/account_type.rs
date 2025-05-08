use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum AccountType {
    Uninitialized,
    StakePool,
    ValidatorList,
}

impl AccountType {
    inherent_borsh_serde!();
}

impl Default for AccountType {
    #[inline]
    fn default() -> Self {
        Self::Uninitialized
    }
}
