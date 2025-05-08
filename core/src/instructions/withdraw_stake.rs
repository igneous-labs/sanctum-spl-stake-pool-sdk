use generic_array_struct::generic_array_struct;

use crate::{StakePool, STAKE_PROGRAM, SYSVAR_CLOCK};

use super::INSTRUCTION_IDX_WITHDRAW_STAKE;

/// The stake_to_receive account must be a rent exempt uninitialized stake account
#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct WithdrawStakeIxAccs<T> {
    pub stake_pool: T,
    pub validator_list: T,
    pub withdraw_auth: T,
    pub stake_to_split: T,
    pub stake_to_receive: T,
    pub user_stake_auth: T,
    pub user_transfer_auth: T,
    pub pool_tokens_from: T,
    pub manager_fee: T,
    pub pool_mint: T,
    pub sysvar_clock: T,
    pub token_program: T,
    pub stake_program: T,
}

pub type WithdrawStakeIxKeysOwned = WithdrawStakeIxAccs<[u8; 32]>;
pub type WithdrawStakeIxKeys<'a> = WithdrawStakeIxAccs<&'a [u8; 32]>;
pub type WithdrawStakeIxAccsFlag = WithdrawStakeIxAccs<bool>;

pub const WITHDRAW_STAKE_IX_PREFIX_IS_WRITER: WithdrawStakeIxAccsFlag =
    WithdrawStakeIxAccs([false; WITHDRAW_STAKE_IX_ACCS_LEN])
        .const_with_stake_pool(true)
        .const_with_validator_list(true)
        .const_with_stake_to_split(true)
        .const_with_stake_to_receive(true)
        .const_with_pool_tokens_from(true)
        .const_with_manager_fee(true)
        .const_with_pool_mint(true);

pub const WITHDRAW_STAKE_IX_PREFIX_IS_SIGNER: WithdrawStakeIxAccsFlag =
    WithdrawStakeIxAccs([false; WITHDRAW_STAKE_IX_ACCS_LEN]).const_with_user_transfer_auth(true);

impl<T: Clone> WithdrawStakeIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; WITHDRAW_STAKE_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl WithdrawStakeIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> WithdrawStakeIxKeys<'_> {
        WithdrawStakeIxKeys::new(self.0.each_ref())
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

impl<'a> WithdrawStakeIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> WithdrawStakeIxKeysOwned {
        WithdrawStakeIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool {
            validator_list,
            manager_fee_account,
            pool_mint,
            token_program_id,
            ..
        }: &'a StakePool,
    ) -> Self {
        self.const_with_validator_list(validator_list)
            .const_with_manager_fee(manager_fee_account)
            .const_with_pool_mint(pool_mint)
            .const_with_token_program(token_program_id)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_stake_program(&STAKE_PROGRAM)
            .const_with_sysvar_clock(&SYSVAR_CLOCK)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawStakeIxData([u8; 9]);

impl WithdrawStakeIxData {
    #[inline]
    pub fn new(pool_tokens_in: u64) -> Self {
        let mut buf = [0u8; 9];

        buf[0] = INSTRUCTION_IDX_WITHDRAW_STAKE;
        buf[1..9].copy_from_slice(&pool_tokens_in.to_le_bytes());

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 9] {
        self.0
    }
}
