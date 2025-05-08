use generic_array_struct::generic_array_struct;

use crate::{StakePool, STAKE_PROGRAM, SYSVAR_CLOCK, SYSVAR_STAKE_HISTORY};

use super::INSTRUCTION_IDX_WITHDRAW_SOL;

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct WithdrawSolIxPrefixAccs<T> {
    pub stake_pool: T,
    pub withdraw_auth: T,
    pub user_transfer_auth: T,
    pub pool_tokens_from: T,
    pub reserve_stake: T,
    pub lamports_to: T,
    pub manager_fee: T,
    pub pool_mint: T,
    pub sysvar_clock: T,
    pub sysvar_stake_history: T,
    pub stake_program: T,
    pub token_program: T,
}

pub type WithdrawSolIxPrefixKeysOwned = WithdrawSolIxPrefixAccs<[u8; 32]>;
pub type WithdrawSolIxPrefixKeys<'a> = WithdrawSolIxPrefixAccs<&'a [u8; 32]>;
pub type WithdrawSolIxPrefixAccsFlag = WithdrawSolIxPrefixAccs<bool>;

pub const WITHDRAW_SOL_IX_PREFIX_IS_WRITER: WithdrawSolIxPrefixAccsFlag =
    WithdrawSolIxPrefixAccs([false; WITHDRAW_SOL_IX_PREFIX_ACCS_LEN])
        .const_with_stake_pool(true)
        .const_with_pool_tokens_from(true)
        .const_with_reserve_stake(true)
        .const_with_lamports_to(true)
        .const_with_manager_fee(true)
        .const_with_pool_mint(true);

pub const WITHDRAW_SOL_IX_PREFIX_IS_SIGNER: WithdrawSolIxPrefixAccsFlag =
    WithdrawSolIxPrefixAccs([false; WITHDRAW_SOL_IX_PREFIX_ACCS_LEN])
        .const_with_user_transfer_auth(true);

impl<T: Clone> WithdrawSolIxPrefixAccs<T> {
    #[inline]
    pub const fn new(arr: [T; WITHDRAW_SOL_IX_PREFIX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl WithdrawSolIxPrefixKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> WithdrawSolIxPrefixKeys<'_> {
        WithdrawSolIxPrefixKeys::new(self.0.each_ref())
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

impl<'a> WithdrawSolIxPrefixKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> WithdrawSolIxPrefixKeysOwned {
        WithdrawSolIxPrefixKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool {
            reserve_stake,
            manager_fee_account,
            pool_mint,
            token_program_id,
            ..
        }: &'a StakePool,
    ) -> Self {
        self.const_with_reserve_stake(reserve_stake)
            .const_with_manager_fee(manager_fee_account)
            .const_with_pool_mint(pool_mint)
            .const_with_token_program(token_program_id)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_stake_program(&STAKE_PROGRAM)
            .const_with_sysvar_stake_history(&SYSVAR_STAKE_HISTORY)
            .const_with_sysvar_clock(&SYSVAR_CLOCK)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawSolIxData([u8; 9]);

impl WithdrawSolIxData {
    #[inline]
    pub fn new(pool_tokens_in: u64) -> Self {
        let mut buf = [0u8; 9];

        buf[0] = INSTRUCTION_IDX_WITHDRAW_SOL;
        buf[1..9].copy_from_slice(&pool_tokens_in.to_le_bytes());

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 9] {
        self.0
    }
}
