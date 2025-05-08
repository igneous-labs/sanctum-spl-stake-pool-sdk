use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub enum FutureEpoch<T> {
    None,
    One(T),
    Two(T),
}

impl<T> Default for FutureEpoch<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T: BorshSerialize> FutureEpoch<T> {
    inherent_borsh_ser!();
}

impl<T: BorshDeserialize> FutureEpoch<T> {
    inherent_borsh_de!();
}
