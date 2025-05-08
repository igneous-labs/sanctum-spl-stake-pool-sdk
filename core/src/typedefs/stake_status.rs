use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum StakeStatus {
    /// Stake account is active, there may be a transient stake as well
    Active,
    /// Only transient stake account exists, when a transient stake is
    /// deactivating during validator removal
    DeactivatingTransient,
    /// No more validator stake accounts exist, entry ready for removal during
    /// `UpdateStakePoolBalance`
    ReadyForRemoval,
    /// Only the validator stake account is deactivating, no transient stake
    /// account exists
    DeactivatingValidator,
    /// Both the transient and validator stake account are deactivating, when
    /// a validator is removed with a transient stake active
    DeactivatingAll,
}

impl StakeStatus {
    inherent_borsh_serde!();
}

impl StakeStatus {
    #[inline]
    pub fn as_byte(&self) -> u8 {
        // unwrap-safety: StakeStatus is a simple 1 byte enum
        const _STAKE_STATUS_IS_ONE_BYTE: () = assert!(core::mem::size_of::<StakeStatus>() == 1);
        let mut res = 0u8;
        self.borsh_ser(core::slice::from_mut(&mut res)).unwrap();
        res
    }
}

impl Default for StakeStatus {
    #[inline]
    fn default() -> Self {
        Self::Active
    }
}
