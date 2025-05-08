use generic_array_struct::generic_array_struct;

use crate::{StakePool, STAKE_PROGRAM, SYSVAR_CLOCK, SYSVAR_STAKE_HISTORY};

use super::INSTRUCTION_IDX_DEPOSIT_STAKE;

#[generic_array_struct(builder pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct DepositStakeIxAccs<T> {
    pub stake_pool: T,
    pub validator_list: T,
    pub deposit_auth: T,
    pub withdraw_auth: T,
    pub deposit_stake: T,
    pub validator_stake: T,
    pub reserve_stake: T,
    pub pool_tokens_to: T,
    pub manager_fee: T,
    pub referral_pool_tokens: T,
    pub pool_mint: T,
    pub sysvar_clock: T,
    pub sysvar_stake_history: T,
    pub token_program: T,
    pub stake_program: T,
}

pub type DepositStakeIxKeysOwned = DepositStakeIxAccs<[u8; 32]>;
pub type DepositStakeIxKeys<'a> = DepositStakeIxAccs<&'a [u8; 32]>;
pub type DepositStakeIxAccsFlag = DepositStakeIxAccs<bool>;

pub const DEPOSIT_STAKE_IX_IS_WRITER: DepositStakeIxAccsFlag =
    DepositStakeIxAccs([false; DEPOSIT_STAKE_IX_ACCS_LEN])
        .const_with_stake_pool(true)
        .const_with_validator_list(true)
        .const_with_deposit_stake(true)
        .const_with_validator_stake(true)
        .const_with_reserve_stake(true)
        .const_with_pool_tokens_to(true)
        .const_with_manager_fee(true)
        .const_with_referral_pool_tokens(true)
        .const_with_pool_mint(true);

pub const DEPOSIT_STAKE_IX_IS_SIGNER: DepositStakeIxAccsFlag =
    DepositStakeIxAccs([false; DEPOSIT_STAKE_IX_ACCS_LEN]);

impl<T: Clone> DepositStakeIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; DEPOSIT_STAKE_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl DepositStakeIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> DepositStakeIxKeys<'_> {
        DepositStakeIxKeys::new(self.0.each_ref())
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

impl<'a> DepositStakeIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> DepositStakeIxKeysOwned {
        DepositStakeIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool {
            validator_list,
            reserve_stake,
            manager_fee_account,
            pool_mint,
            token_program_id,
            ..
        }: &'a StakePool,
    ) -> Self {
        self.const_with_validator_list(validator_list)
            .const_with_reserve_stake(reserve_stake)
            .const_with_manager_fee(manager_fee_account)
            .const_with_pool_mint(pool_mint)
            .const_with_token_program(token_program_id)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_sysvar_clock(&SYSVAR_CLOCK)
            .const_with_sysvar_stake_history(&SYSVAR_STAKE_HISTORY)
            .const_with_stake_program(&STAKE_PROGRAM)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositStakeIxData([u8; 1]);

impl DepositStakeIxData {
    #[inline]
    pub const fn new() -> Self {
        Self([INSTRUCTION_IDX_DEPOSIT_STAKE])
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 1] {
        self.0
    }
}
