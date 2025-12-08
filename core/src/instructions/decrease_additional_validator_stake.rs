use generic_array_struct::generic_array_struct;

use crate::{
    StakePool, INSTRUCTION_IDX_DECREASE_ADDITIONAL_VALIDATOR_STAKE, STAKE_PROGRAM, SYSTEM_PROGRAM,
    SYSVAR_CLOCK, SYSVAR_STAKE_HISTORY,
};

#[generic_array_struct(builder pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct DecreaseAdditionalValidatorStakeIxAccs<T> {
    pub stake_pool: T,
    pub staker: T,
    pub withdraw_auth: T,
    pub validator_list: T,
    pub reserve: T,
    pub validator_stake: T,
    pub ephemeral_stake: T,
    pub transient_stake: T,
    pub sysvar_clock: T,
    pub sysvar_stake_history: T,
    pub system_program: T,
    pub stake_program: T,
}

pub type DecreaseAdditionalValidatorStakeIxKeysOwned =
    DecreaseAdditionalValidatorStakeIxAccs<[u8; 32]>;
pub type DecreaseAdditionalValidatorStakeIxKeys<'a> =
    DecreaseAdditionalValidatorStakeIxAccs<&'a [u8; 32]>;
pub type DecreaseAdditionalValidatorStakeIxAccsFlag = DecreaseAdditionalValidatorStakeIxAccs<bool>;

pub const DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_IS_WRITER:
    DecreaseAdditionalValidatorStakeIxAccsFlag = DecreaseAdditionalValidatorStakeIxAccs(
    [false; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCS_LEN],
)
.const_with_validator_list(true)
.const_with_reserve(true)
.const_with_validator_stake(true)
.const_with_ephemeral_stake(true)
.const_with_transient_stake(true);

pub const DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_IS_SIGNER:
    DecreaseAdditionalValidatorStakeIxAccsFlag = DecreaseAdditionalValidatorStakeIxAccs(
    [false; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCS_LEN],
)
.const_with_staker(true);

impl<T: Clone> DecreaseAdditionalValidatorStakeIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl DecreaseAdditionalValidatorStakeIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> DecreaseAdditionalValidatorStakeIxKeys<'_> {
        DecreaseAdditionalValidatorStakeIxKeys::new(self.0.each_ref())
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

impl<'a> DecreaseAdditionalValidatorStakeIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> DecreaseAdditionalValidatorStakeIxKeysOwned {
        DecreaseAdditionalValidatorStakeIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        StakePool {
            reserve_stake,
            validator_list,
            staker,
            ..
        }: &'a StakePool,
    ) -> Self {
        self.const_with_reserve(reserve_stake)
            .const_with_validator_list(validator_list)
            .const_with_staker(staker)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_sysvar_clock(&SYSVAR_CLOCK)
            .const_with_sysvar_stake_history(&SYSVAR_STAKE_HISTORY)
            .const_with_system_program(&SYSTEM_PROGRAM)
            .const_with_stake_program(&STAKE_PROGRAM)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecreaseAdditionalValidatorStakeIxData([u8; 25]);

impl DecreaseAdditionalValidatorStakeIxData {
    #[inline]
    pub fn new(lamports: u64, transient_stake_seed: u64) -> Self {
        let mut buf = [0u8; 25];

        buf[0] = INSTRUCTION_IDX_DECREASE_ADDITIONAL_VALIDATOR_STAKE;
        buf[1..9].copy_from_slice(&lamports.to_le_bytes());
        buf[9..17].copy_from_slice(&transient_stake_seed.to_le_bytes());
        // ephemeral seed
        buf[17..25].copy_from_slice(&[0u8; 8]);

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 25] {
        self.0
    }
}
