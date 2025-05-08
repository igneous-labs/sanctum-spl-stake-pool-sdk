use generic_array_struct::generic_array_struct;

use crate::StakePool;

use super::INSTRUCTION_IDX_UPDATE_STAKE_POOL_BALANCE;

#[generic_array_struct(builder pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct UpdateStakePoolBalanceIxAccs<T> {
    pub stake_pool: T,
    pub withdraw_auth: T,
    pub validator_list: T,
    pub reserve: T,
    pub manager_fee: T,
    pub pool_mint: T,
    pub pool_token_prog: T,
}

pub type UpdateStakePoolBalanceIxKeysOwned = UpdateStakePoolBalanceIxAccs<[u8; 32]>;
pub type UpdateStakePoolBalanceIxKeys<'a> = UpdateStakePoolBalanceIxAccs<&'a [u8; 32]>;
pub type UpdateStakePoolBalanceIxAccsFlag = UpdateStakePoolBalanceIxAccs<bool>;

pub const UPDATE_STAKE_POOL_BALANCE_IX_IS_WRITER: UpdateStakePoolBalanceIxAccsFlag =
    UpdateStakePoolBalanceIxAccs([false; UPDATE_STAKE_POOL_BALANCE_IX_ACCS_LEN])
        .const_with_stake_pool(true)
        .const_with_validator_list(true)
        .const_with_manager_fee(true)
        .const_with_pool_mint(true);

pub const UPDATE_STAKE_POOL_BALANCE_IX_IS_SIGNER: UpdateStakePoolBalanceIxAccsFlag =
    UpdateStakePoolBalanceIxAccs([false; UPDATE_STAKE_POOL_BALANCE_IX_ACCS_LEN]);

impl UpdateStakePoolBalanceIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> UpdateStakePoolBalanceIxKeys<'_> {
        UpdateStakePoolBalanceIxKeys::new(self.0.each_ref())
    }

    #[inline]
    pub fn with_keys_from_stake_pool(self, pool: &StakePool) -> Self {
        self.as_borrowed()
            .with_keys_from_stake_pool(pool)
            .into_owned()
    }
}

impl<'a> UpdateStakePoolBalanceIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> UpdateStakePoolBalanceIxKeysOwned {
        UpdateStakePoolBalanceIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool {
            validator_list,
            reserve_stake,
            pool_mint,
            manager_fee_account,
            token_program_id,
            ..
        }: &'a StakePool,
    ) -> Self {
        self.const_with_manager_fee(manager_fee_account)
            .const_with_pool_mint(pool_mint)
            .const_with_pool_token_prog(token_program_id)
            .const_with_reserve(reserve_stake)
            .const_with_validator_list(validator_list)
    }
}

impl<T> UpdateStakePoolBalanceIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; UPDATE_STAKE_POOL_BALANCE_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateStakePoolBalanceIxData([u8; 1]);

impl UpdateStakePoolBalanceIxData {
    #[inline]
    pub const fn new() -> Self {
        Self([INSTRUCTION_IDX_UPDATE_STAKE_POOL_BALANCE])
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 1] {
        self.0
    }
}
