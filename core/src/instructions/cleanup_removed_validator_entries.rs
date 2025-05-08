use generic_array_struct::generic_array_struct;

use crate::StakePool;

use super::INSTRUCTION_IDX_CLEANUP_REMOVED_VALIDATOR_ENTRIES;

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct CleanupRemovedValidatorEntriesIxAccs<T> {
    pub stake_pool: T,
    pub validator_list: T,
}

pub type CleanupRemovedValidatorEntriesIxKeysOwned = CleanupRemovedValidatorEntriesIxAccs<[u8; 32]>;
pub type CleanupRemovedValidatorEntriesIxKeys<'a> =
    CleanupRemovedValidatorEntriesIxAccs<&'a [u8; 32]>;
pub type CleanupRemovedValidatorEntriesIxAccsFlag = CleanupRemovedValidatorEntriesIxAccs<bool>;

pub const CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_IS_WRITER: CleanupRemovedValidatorEntriesIxAccsFlag =
    CleanupRemovedValidatorEntriesIxAccsFlag::new(
        [false; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCS_LEN],
    )
    .const_with_validator_list(true);

pub const CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_IS_SIGNER: CleanupRemovedValidatorEntriesIxAccsFlag =
    CleanupRemovedValidatorEntriesIxAccsFlag::new(
        [false; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCS_LEN],
    );

impl<T> CleanupRemovedValidatorEntriesIxAccs<T> {
    /// This seems redundant because `.0` is `pub`, but this is necessary for
    /// nice init syntax with type aliases.
    ///
    /// With this, you can now do
    ///
    /// ```
    /// use sanctum_spl_stake_pool_core::CleanupRemovedValidatorEntriesIxAccsFlag;
    /// let var: CleanupRemovedValidatorEntriesIxAccsFlag = CleanupRemovedValidatorEntriesIxAccsFlag::new(Default::default());
    /// ```
    ///
    /// instead of
    ///
    /// ```
    /// use sanctum_spl_stake_pool_core::{CleanupRemovedValidatorEntriesIxAccsFlag, CleanupRemovedValidatorEntriesIxAccs};
    /// let var: CleanupRemovedValidatorEntriesIxAccsFlag = CleanupRemovedValidatorEntriesIxAccs(Default::default());
    /// ```
    #[inline]
    pub const fn new(arr: [T; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl<'a> CleanupRemovedValidatorEntriesIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> CleanupRemovedValidatorEntriesIxKeysOwned {
        CleanupRemovedValidatorEntriesIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool { validator_list, .. }: &'a StakePool,
    ) -> Self {
        self.const_with_validator_list(validator_list)
    }
}

impl CleanupRemovedValidatorEntriesIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> CleanupRemovedValidatorEntriesIxKeys<'_> {
        CleanupRemovedValidatorEntriesIxKeys::new(self.0.each_ref())
    }

    #[inline]
    pub fn with_keys_from_stake_pool(self, pool: &StakePool) -> Self {
        self.as_borrowed()
            .with_keys_from_stake_pool(pool)
            .into_owned()
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanupRemovedValidatorEntriesIxData([u8; 1]);

impl CleanupRemovedValidatorEntriesIxData {
    #[inline]
    pub const fn new() -> Self {
        Self([INSTRUCTION_IDX_CLEANUP_REMOVED_VALIDATOR_ENTRIES])
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 1] {
        self.0
    }
}
