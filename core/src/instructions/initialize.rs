use generic_array_struct::generic_array_struct;

use super::INSTRUCTION_IDX_INITIALIZE;
use crate::Fee;

#[generic_array_struct(builder pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct InitializeIxPrefixAccs<T> {
    pub stake_pool: T,
    pub manager: T,
    pub staker: T,
    pub withdraw_auth: T,
    pub validator_list: T,
    pub reserve: T,
    pub pool_mint: T,
    pub manager_fee: T,
    pub pool_token_prog: T,
}

pub type InitializeIxPrefixKeysOwned = InitializeIxPrefixAccs<[u8; 32]>;
pub type InitializeIxPrefixKeys<'a> = InitializeIxPrefixAccs<&'a [u8; 32]>;
pub type InitializeIxPrefixAccsFlag = InitializeIxPrefixAccs<bool>;

pub const INITIALIZE_IX_PREFIX_IS_WRITER: InitializeIxPrefixAccsFlag =
    InitializeIxPrefixAccs([false; INITIALIZE_IX_PREFIX_ACCS_LEN])
        .const_with_stake_pool(true)
        .const_with_validator_list(true)
        .const_with_pool_mint(true)
        .const_with_manager_fee(true);

pub const INITIALIZE_IX_PREFIX_IS_SIGNER: InitializeIxPrefixAccsFlag =
    InitializeIxPrefixAccs([false; INITIALIZE_IX_PREFIX_ACCS_LEN]).const_with_manager(true);

impl<T: Clone> InitializeIxPrefixAccs<T> {
    #[inline]
    pub const fn new(arr: [T; INITIALIZE_IX_PREFIX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl InitializeIxPrefixKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> InitializeIxPrefixKeys<'_> {
        InitializeIxPrefixKeys::new(self.0.each_ref())
    }
}

impl InitializeIxPrefixKeys<'_> {
    #[inline]
    pub fn into_owned(self) -> InitializeIxPrefixKeysOwned {
        InitializeIxPrefixKeysOwned::new(self.0.map(|pk| *pk))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxData(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] [u8; 54]);

impl InitializeIxData {
    #[inline]
    pub fn new(
        fee: Fee,
        withdrawal_fee: Fee,
        deposit_fee: Fee,
        referral_fee: u8,
        max_validators: u32,
    ) -> Self {
        let mut buf = [0u8; 54];

        buf[0] = INSTRUCTION_IDX_INITIALIZE;
        buf[1..9].copy_from_slice(&fee.denominator.to_le_bytes());
        buf[9..17].copy_from_slice(&fee.numerator.to_le_bytes());
        buf[17..25].copy_from_slice(&withdrawal_fee.denominator.to_le_bytes());
        buf[25..33].copy_from_slice(&withdrawal_fee.numerator.to_le_bytes());
        buf[33..41].copy_from_slice(&deposit_fee.denominator.to_le_bytes());
        buf[41..49].copy_from_slice(&deposit_fee.numerator.to_le_bytes());
        buf[49] = referral_fee;
        buf[50..54].copy_from_slice(&max_validators.to_le_bytes());

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 54] {
        self.0
    }
}
