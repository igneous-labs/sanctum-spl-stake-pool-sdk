use generic_array_struct::generic_array_struct;

use super::INSTRUCTION_IDX_UPDATE_VALIDATOR_LIST_BALANCE;
use crate::{StakePool, STAKE_PROGRAM, SYSVAR_CLOCK, SYSVAR_STAKE_HISTORY};

#[generic_array_struct(builder pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct UpdateValidatorListBalanceIxPrefixAccs<T> {
    pub stake_pool: T,
    pub withdraw_auth: T,
    pub validator_list: T,
    pub reserve: T,
    pub sysvar_clock: T,
    pub sysvar_stake_history: T,
    pub stake_program: T,
}

pub type UpdateValidatorListBalanceIxPrefixKeysOwned =
    UpdateValidatorListBalanceIxPrefixAccs<[u8; 32]>;
pub type UpdateValidatorListBalanceIxPrefixKeys<'a> =
    UpdateValidatorListBalanceIxPrefixAccs<&'a [u8; 32]>;
pub type UpdateValidatorListBalanceIxPrefixAccsFlag = UpdateValidatorListBalanceIxPrefixAccs<bool>;

pub const UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_IS_WRITER:
    UpdateValidatorListBalanceIxPrefixAccsFlag = UpdateValidatorListBalanceIxPrefixAccs(
    [false; UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_ACCS_LEN],
)
.const_with_validator_list(true)
.const_with_reserve(true);

pub const UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_IS_SIGNER:
    UpdateValidatorListBalanceIxPrefixAccsFlag = UpdateValidatorListBalanceIxPrefixAccs(
    [false; UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_ACCS_LEN],
);

impl<T: Clone> UpdateValidatorListBalanceIxPrefixAccs<T> {
    #[inline]
    pub const fn new(arr: [T; UPDATE_VALIDATOR_LIST_BALANCE_IX_PREFIX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl UpdateValidatorListBalanceIxPrefixKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> UpdateValidatorListBalanceIxPrefixKeys<'_> {
        UpdateValidatorListBalanceIxPrefixKeys::new(self.0.each_ref())
    }

    #[inline]
    pub fn with_keys_from_stake_pool(self, pool: &StakePool) -> Self {
        self.as_borrowed()
            .with_keys_from_stake_pool(pool)
            .into_owned()
    }

    #[inline]
    pub fn with_consts(self) -> Self {
        self.as_borrowed().with_consts().into_owned()
    }
}

impl<'a> UpdateValidatorListBalanceIxPrefixKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> UpdateValidatorListBalanceIxPrefixKeysOwned {
        UpdateValidatorListBalanceIxPrefixKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool {
            validator_list,
            reserve_stake,
            ..
        }: &'a StakePool,
    ) -> Self {
        self.const_with_reserve(reserve_stake)
            .const_with_validator_list(validator_list)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_stake_program(&STAKE_PROGRAM)
            .const_with_sysvar_clock(&SYSVAR_CLOCK)
            .const_with_sysvar_stake_history(&SYSVAR_STAKE_HISTORY)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateValidatorListBalanceIxData([u8; 6]);

impl UpdateValidatorListBalanceIxData {
    #[inline]
    pub fn new(start_index: u32, no_merge: bool) -> Self {
        let mut buf = [0u8; 6];

        buf[0] = INSTRUCTION_IDX_UPDATE_VALIDATOR_LIST_BALANCE;
        buf[1..5].copy_from_slice(&start_index.to_le_bytes());
        buf[5] = no_merge.into();

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 6] {
        self.0
    }
}
