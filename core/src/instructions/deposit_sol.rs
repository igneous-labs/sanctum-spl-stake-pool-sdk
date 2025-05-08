use generic_array_struct::generic_array_struct;

use crate::{StakePool, SYSTEM_PROGRAM};

use super::INSTRUCTION_IDX_DEPOSIT_SOL;

/// If the pool has a non-default sol deposit authority, then the following
/// accounts follow after this prefix:
///
///  - `[s]`  sol deposit authority
///
/// Otherwise, this is the full instruction accounts array
#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct DepositSolIxPrefixAccs<T> {
    pub stake_pool: T,
    pub withdraw_auth: T,
    pub reserve: T,
    pub from_user_lamports: T,
    pub dest_user_pool: T,
    pub manager_fee: T,
    pub referrer_fee: T,
    pub pool_mint: T,
    pub system_program: T,
    pub token_program: T,
}

pub type DepositSolIxPrefixKeysOwned = DepositSolIxPrefixAccs<[u8; 32]>;
pub type DepositSolIxPrefixKeys<'a> = DepositSolIxPrefixAccs<&'a [u8; 32]>;
pub type DepositSolIxPrefixAccsFlag = DepositSolIxPrefixAccs<bool>;

pub const DEPOSIT_SOL_IX_PREFIX_IS_WRITER: DepositSolIxPrefixAccsFlag =
    DepositSolIxPrefixAccs([false; DEPOSIT_SOL_IX_PREFIX_ACCS_LEN])
        .const_with_stake_pool(true)
        .const_with_reserve(true)
        .const_with_from_user_lamports(true)
        .const_with_dest_user_pool(true)
        .const_with_manager_fee(true)
        .const_with_referrer_fee(true)
        .const_with_pool_mint(true);

pub const DEPOSIT_SOL_IX_PREFIX_IS_SIGNER: DepositSolIxPrefixAccsFlag =
    DepositSolIxPrefixAccs([false; DEPOSIT_SOL_IX_PREFIX_ACCS_LEN])
        .const_with_from_user_lamports(true);

impl<T: Clone> DepositSolIxPrefixAccs<T> {
    #[inline]
    pub const fn new(arr: [T; DEPOSIT_SOL_IX_PREFIX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl DepositSolIxPrefixKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> DepositSolIxPrefixKeys<'_> {
        DepositSolIxPrefixKeys::new(self.0.each_ref())
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

impl<'a> DepositSolIxPrefixKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> DepositSolIxPrefixKeysOwned {
        DepositSolIxPrefixKeysOwned::new(self.0.map(|pk| *pk))
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
        self.const_with_reserve(reserve_stake)
            .const_with_manager_fee(manager_fee_account)
            .const_with_pool_mint(pool_mint)
            .const_with_token_program(token_program_id)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_system_program(&SYSTEM_PROGRAM)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositSolIxData([u8; 9]);

impl DepositSolIxData {
    #[inline]
    pub fn new(deposit_lamports: u64) -> Self {
        let mut buf = [0u8; 9];

        buf[0] = INSTRUCTION_IDX_DEPOSIT_SOL;
        buf[1..9].copy_from_slice(&deposit_lamports.to_le_bytes());

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 9] {
        self.0
    }
}
